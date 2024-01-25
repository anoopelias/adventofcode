import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("src/2021/22/input");
  const splits = input.toString().split("\n");
  splits.pop();

  const rows: Row[] = [];
  const xBlocks: number[] = [];
  const yBlocks: number[] = [];
  const zBlocks: number[] = [];

  for (let split of splits) {
    const rowValues = split.split(" ");
    const operation = rowValues[0];
    const coordRanges: [number, number][] = rowValues[1].split(",")
      .map(coordRangeStr => {
        const [min, max] = coordRangeStr.split("=")[1].split("..")
          .map(s => parseInt(s));
        return [min, max];
      });
    coordRanges[0][1]++;
    coordRanges[1][1]++;
    coordRanges[2][1]++;
    xBlocks.push(coordRanges[0][0], coordRanges[0][1]);
    yBlocks.push(coordRanges[1][0], coordRanges[1][1]);
    zBlocks.push(coordRanges[2][0], coordRanges[2][1]);
    rows.push(new Row(operation, coordRanges[0], coordRanges[1], coordRanges[2]))
  }

  xBlocks.sort((a, b) => a - b);
  yBlocks.sort((a, b) => a - b);
  zBlocks.sort((a, b) => a - b);

  console.log(xBlocks.length, yBlocks.length, zBlocks.length);

  const cubes: boolean[][][] = [];
  for (let i = 0; i < xBlocks.length; i++) {
    cubes.push([]);
    for (let j = 0; j < yBlocks.length; j++) {
      cubes[i].push([]);
      for (let k = 0; k < zBlocks.length; k++) {
        cubes[i][j].push(false);
      }
    }
  }

  for (let i = 0; i < rows.length; i++) {
    let row = rows[i];
    const isOn = row.operation === "on";
    for (let i = 0; i < xBlocks.length; i++) {
      if (xBlocks[i] >= row.range.x[0] && xBlocks[i] < row.range.x[1]) {
        for (let j = 0; j < yBlocks.length; j++) {
          if (yBlocks[j] >= row.range.y[0] && yBlocks[j] < row.range.y[1]) {
            for (let k = 0; k < zBlocks.length; k++) {
              if (zBlocks[k] >= row.range.z[0] && zBlocks[k] < row.range.z[1]) {
                cubes[i][j][k] = isOn;
              }
            }  
          }
        }
      }
    }  
  }

  let count = BigInt(0);
  for (let i = 0; i < xBlocks.length - 1; i++) {
    for (let j = 0; j < yBlocks.length - 1; j++) {
      for (let k = 0; k < zBlocks.length - 1; k++) {
        if (cubes[i][j][k]) {
          const x = xBlocks[i +  1] - xBlocks[i];
          const y = yBlocks[j +  1] - yBlocks[j];
          const z = zBlocks[k +  1] - zBlocks[k];
          count += (BigInt(x) * BigInt(y) * BigInt(z));
        }
      }
    }
  }

  console.log(count);
}


class Range {
  x: [number, number];
  y: [number, number];
  z: [number, number];

  constructor(x: [number, number], y: [number, number], z: [number, number]) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  magnitude(): bigint {
    const xlen = (this.x[1] - this.x[0] + 1);
    const ylen = (this.x[1] - this.y[0] + 1);
    const zlen = (this.x[1] - this.z[0] + 1);

    return BigInt(xlen) * BigInt(ylen) * BigInt(zlen);
  }

}

class Row {
  operation: string;
  range: Range;

  constructor(operation: string, x: [number, number], y: [number, number], z: [number, number]) {
    this.operation = operation;
    this.range = new Range(x, y, z);
  }
}

// 1130514303649907n
solve();
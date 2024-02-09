import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("../../../aoc-files/2021/22/input3");
  const splits = input.toString().split("\n");
  splits.pop();

  const rows: Row[] = [];
  for (let split of splits) {
    const rowValues = split.split(" ");
    const operation = rowValues[0];
    const coordRanges: [number, number][] = rowValues[1].split(",")
      .map(coordRangeStr => {
        const [min, max] = coordRangeStr.split("=")[1].split("..")
          .map(s => parseInt(s));
        return [min, max];
      });
    rows.push(new Row(operation, coordRanges[0], coordRanges[1], coordRanges[2]))
  }

  let cuboid: boolean[][][] = [];
  for (let i = -50; i <= 50; i++) {
    cuboid[i] = [];
    for (let j = -50; j <= 50; j++) {
      cuboid[i][j] = [];
      for (let k = -50; k <= 50; k++) {
        cuboid[i][j][k] = false;
      }
    }
  }

  for (let row of rows) {
    const xStart = Math.max(-50, row.x[0]);
    const xEnd = Math.min(50, row.x[1]);
    const yStart = Math.max(-50, row.y[0]);
    const yEnd = Math.min(50, row.y[1]);
    const zStart = Math.max(-50, row.z[0]);
    const zEnd = Math.min(50, row.z[1]);

    for (let i = xStart; i <= xEnd; i++) {
      for (let j = yStart; j <= yEnd; j++) {
        for (let k = zStart; k <= zEnd; k++) {
          cuboid[i][j][k] = row.operation === "on" ? true : false;
        }
      }
    }
  }


  let count = 0;
  for (let i = -50; i <= 50; i++) {
    for (let j = -50; j <= 50; j++) {
      for (let k = -50; k <= 50; k++) {
        if (cuboid[i][j][k]) count++;
      }
    }
  }
  console.log(count);
}

class Row {
  operation: string;
  x: [number, number];
  y: [number, number];
  z: [number, number];

  constructor(operation: string, x: [number, number], y: [number, number], z: [number, number]) {
    this.operation = operation;
    this.x = x;
    this.y = y;
    this.z = z;
  }
}

solve();
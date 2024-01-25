import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("src/2021/19/input");
  const splits = input.toString().split("\n");
  splits.pop();

  const scanners: Scanner[] = [];
  let scannerNum = 0;
  let scanner: Scanner;

  for (let split of splits) {
    if (split.indexOf("scanner") !== -1) {
      scanner = new Scanner(scannerNum);
      scanners.push(scanner);
      scannerNum++;
    } else if (split.length !== 0) {
      let [x, y, z] = split.split(",").map(s => parseInt(s));
      scanner!.readings.push([x, y, z]);
    }
  }

  scanners[0].rpos = [0, 0, 0];
  scanners[0].transformation = new Transformation([0, 1, 2], [1, 1, 1]);
  const transformations = getAllTransformations();
  let count = 1;

  while (count < scanners.length) {
    for (let i = 0; i < scanners.length; i++) {
      for (let j = 0; j < scanners.length; j++) {
        if (scanners[i].rpos !== null && scanners[j].rpos === null) {
          getRelativePos(scanners[i], scanners[j], transformations);
          if (scanners[j].rpos !== null) {
            count++;
          }
        }
      }
    }  
  }

  const beacons: Set<string> = new Set<string>();
  for (let i = 0; i < scanners.length; i++) {
    for (let reading of scanners[i].readings) {
      beacons.add(toString(scanners[i].transform(reading)));
    }
  }

  let largestManhattan = 0;
  for (let i = 0; i < scanners.length; i++) {
    for (let j = i; j < scanners.length; j++) {
      const iPos = scanners[i].rpos!;
      const jPos = scanners[j].rpos!;
      const dist = Math.abs(iPos[0] - jPos[0]) + Math.abs(iPos[1] - jPos[1]) + Math.abs(iPos[2] - jPos[2]);
      largestManhattan = Math.max(largestManhattan, dist);
    }
  }

  console.log(beacons.size);
  console.log(largestManhattan);
}

function getRelativePos(scanner1: Scanner, scanner2: Scanner, transformations: Transformation[]) {
  const relativePosMaps: Map<string, number>[] = [];

  for (let i = 0; i < transformations.length; i++) {
    const transformation = transformations[i];
    const relativePosMap = new Map<string, number>();
    for (let s1Reading of scanner1.readings) {
      s1Reading = scanner1.transform(s1Reading);
      for (let s2Reading of scanner2.readings) {
        const relativeScannerPos = getRelativeScannerPos(s1Reading, s2Reading, transformation);
        const key = toString(relativeScannerPos);
        if (!relativePosMap.has(key)) {
          relativePosMap.set(key, 0);
        }
        relativePosMap.set(key, relativePosMap.get(key)! + 1);
      }
    }
    relativePosMaps.push(relativePosMap);
  }

  for (let i = 0; i < transformations.length; i++) {
    const relativePosMap = relativePosMaps[i];
    const transformation = transformations[i];
    for (let [key, value] of relativePosMap) {
      if (value >= 12) {
        scanner2.rpos = fromString(key);
        scanner2.transformation = transformation;
        break;
      }
    }
  }

}

function getRelativeScannerPos(coord1: Coords, coord2: Coords, transformation: Transformation): Coords {
  coord2 = transformation.transform(coord2);
  return [
    coord1[0] - coord2[0],
    coord1[1] - coord2[1],
    coord1[2] - coord2[2],
  ];
}

function getAllTransformations() {
  const rotations = getAllRotations();
  const multiplications = getAllMultiplications();
  const transformations: Transformation[] = [];

  for (let rotation of rotations) {
    for (let multiplication of multiplications) {
      // Actually only half of the transformations combinations are feasible.
      // But this seems to work.
      transformations.push(new Transformation(rotation, multiplication));
    }
  }

  return transformations;
}

function getAllRotations(): [number, number, number][] {
  return [
    [0, 1, 2],
    [0, 2, 1],
    [1, 2, 0],
    [1, 0, 2],
    [2, 0, 1],
    [2, 1, 0],
  ]
}

function getAllMultiplications(): [number, number, number][] {
  return [
    [1, 1, 1],
    [1, 1, -1],
    [1, -1, 1],
    [1, -1, -1],
    [-1, 1, 1],
    [-1, 1, -1],
    [-1, -1, 1],
    [-1, -1, -1],
  ]
}

function toString(coord: Coords): string {
  return coord[0] + ":" + coord[1] + ":" + coord[2];
}

function fromString(str: string): Coords {
  let [x, y, z] = str.split(":").map(s => parseInt(s));
  return [x, y, z];
}

class Transformation {
  rotation: [number, number, number];
  multiplication: [number, number, number];

  constructor(rotation: [number, number, number],
    multiplication: [number, number, number]) {

    this.rotation = rotation;
    this.multiplication = multiplication;
  }

  transform(coords: Coords): Coords {
    return [
      coords[this.rotation[0]] * this.multiplication[0],
      coords[this.rotation[1]] * this.multiplication[1],
      coords[this.rotation[2]] * this.multiplication[2]
    ]
  }
}

class Scanner {
  readings: Coords[];
  rpos: Coords | null;
  transformation: Transformation | null;
  num: number;

  constructor(num: number) {
    this.readings = [];
    this.num = num;
    this.rpos = null;
    this.transformation = null;
  }

  transform(coord: Coords): Coords {
    const tCoord = this.transformation!.transform(coord);
    return [
      this.rpos![0] + tCoord[0],
      this.rpos![1] + tCoord[1],
      this.rpos![2] + tCoord[2],
    ]
  }
}

type Coords = [number, number, number];
solve();
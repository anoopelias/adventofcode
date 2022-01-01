import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("src/2021/25/input");
  const splits = input.toString().split("\n");
  splits.pop();

  const map: string[][] = [];

  for (let split of splits) {
    map.push(split.split(""));
  }

  let steps = 0;
  let changed = true;
  while(changed) {
    steps++;
    changed = step(map);
  }
  console.log(steps);
}

function print(map: string[][]) {
  for (let row of map) {
    console.log(row.join(""));
  }
  console.log();
}

function step(map: string[][]): boolean {
  const east = moveEast(map);
  const south = moveSouth(map);
  return east || south;
}

function moveEast(map: string[][]): boolean {
  const [m, n] = [map.length, map[0].length];
  const moves: [number, number][] = [];

  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      if (map[i][j] === ">") {
        let q = j + 1;
        if (q === n) q = 0;
        if (map[i][q] === ".") {
          moves.push([i, j]);
        }
      }
    }
  }

  for (let [i, j] of moves) {
    let q = j + 1;
    if (q === n) q = 0;
    map[i][j] = ".";
    map[i][q] = ">";
  }

  return moves.length > 0;
}

function moveSouth(map: string[][]): boolean {
  const [m, n] = [map.length, map[0].length];
  const moves: [number, number][] = [];

  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      if (map[i][j] === "v") {
        let p = i + 1;
        if (p === m) p = 0;
        if (map[p][j] === ".") {
          moves.push([i, j]);
        }
      }
    }
  }

  for (let [i, j] of moves) {
    let p = i + 1;
    if (p === m) p = 0;
    map[i][j] = ".";
    map[p][j] = "v";
  }
  return moves.length > 0;
}


solve();
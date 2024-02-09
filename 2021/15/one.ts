import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("../../../aoc-files/2021/15/input");
  const splits = input.toString().split("\n");
  splits.pop();
  const density: number[][] = [];
  const distMap: number[][] = [];

  for (let row of splits) {
    density.push(row.split("").map(s => parseInt(s)));
  }
  let [m, n] = [density.length, density[0].length];

  for (let i = 0; i < m; i++) {
    for (let k = 0; k < 4; k++) {
      for (let j = 0; j < n; j++) {
        const index = (k * n) + j;
        let num = density[i][index] + 1;
        if (num === 10) num = 1;
        density[i].push(num);
      }
    }
  }

  n = density[0].length;
  for (let k = 0; k < 4; k++) {
    for (let i = 0; i < m; i++) {
      const row: number[] = [];
      for (let j = 0; j < n; j++) {
        const index = (k * m) + i;
        let num = density[index][j] + 1;
        if (num === 10) num = 1;
        row.push(num);
      }
      density.push(row);
    }
  }

  m = density.length;

  for (let row of density) {
    distMap.push(new Array(n).fill(Infinity));
  }
  const minPQ = new MinPQ();
  minPQ.insert(0, [0, 0]);
  distMap[0][0] = 0;

  while (!minPQ.isEmpty()) {
    const [dist, [p, q]] = minPQ.delMin();
    const neighbors = getNeighbors(p, q, m, n);
    for (let [u, v] of neighbors) {
      const newDist = dist + density[u][v];
      if (newDist < distMap[u][v]) {
        distMap[u][v] = newDist;
        minPQ.insert(newDist, [u, v]);
      }
    }
  }

  console.log(distMap[m - 1][n - 1]);
}

function getNeighbors(i: number, j: number, m: number, n: number): number[][] {
  return [
    [i - 1, j],
    [i, j - 1],
    [i, j + 1],
    [i + 1, j]
  ].filter(([x, y]) => x >= 0 && x < m && y >= 0 && y < n);
}

class MinPQ {
  private arr: [number, [number, number]][] = [[-1, [-1, -1]]];

  insert(n: number, [i, j]: [number, number]) {
    this.arr.push([n, [i, j]]);
    this.swim(this.arr.length - 1);
  }

  delMin(): [number, [number, number]] {
    const max = this.arr[1];
    const newVal = this.arr.pop();

    if (!this.isEmpty()) {
      this.arr[1] = newVal!;
      this.sink(1);
    }

    return max;
  }

  isEmpty(): boolean {
    return this.arr.length === 1;
  }

  min(): [number, [number, number]] {
    return this.arr[1];
  }

  size(): number {
    return this.arr.length - 1;
  }

  private swim(n: number) {
    if (n === 1) return;
    const parent = Math.floor(n / 2);

    const arr = this.arr;
    if (arr[n][0] < arr[parent][0]) {
      [arr[n], arr[parent]] = [arr[parent], arr[n]];
      this.swim(parent);
    }
  }

  private sink(n: number) {
    const arr = this.arr;
    const left = n * 2;
    const right = left + 1;

    if (left >= arr.length) return;

    const leaf = (right < arr.length && arr[right][0] < arr[left][0]) ? right : left;

    if (arr[n][0] <= arr[leaf][0]) return;

    [arr[n], arr[leaf]] = [arr[leaf], arr[n]];
    this.sink(leaf);
  }
}

solve();
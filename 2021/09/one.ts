import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2021/09/input");
    const splits = input.toString().split("\n");
    const heights: number[][] = [];

    for (let split of splits) {
        heights.push(split.split("").map(s => parseInt(s)));
    }

    const [m, n] = [heights.length, heights[0].length];
    const basinSizes: number[] =  [];

    let sum = 0;
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            const neighbors = getNeighbors(i, j, m, n);
            let low = true;
            for (let [p, q] of neighbors) {
                if (heights[p][q] <= heights[i][j]) {
                    low = false;
                    break;
                }
                
            }

            if (low) {
                basinSizes.push(basinSize(i, j, heights));
            }
        }
    }
    basinSizes.sort((a, b) => b - a);
    console.log(basinSizes[0] * basinSizes[1] * basinSizes[2]);
}

function basinSize(i: number, j: number, heights: number[][]) {
    const [m, n] = [heights.length, heights[0].length];
    const queue: [number, number][] = [[i, j]];
    const marked = new Set([i + ":" + j]);
    let size = 0;

    while(queue.length) {
        size++;
        const [p, q] = queue.shift()!;
        const neighbors = getNeighbors(p, q, m, n);

        for (let [u, v] of neighbors) {
            if (heights[u][v] !== 9 && !marked.has(u + ":" + v) && heights[u][v] > heights[i][j]) {
                marked.add(u + ":" + v);
                queue.push([u, v]);
            }

        }
    }
    return size;
}

function getNeighbors(i: number, j: number, m: number, n: number): number[][] {
    return [
        [i - 1, j],
        [i, j - 1],
        [i, j + 1],
        [i + 1, j]
    ].filter(([x, y]) => x >= 0 && x < m && y >= 0 && y < n);
}

solve();
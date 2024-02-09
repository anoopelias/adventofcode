import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2021/11/input");
    const splits = input.toString().split("\n");
    const array: number[][] = [];
    for (let split of splits) {
        array.push(split.split("").map(s => parseInt(s)));
    }

    let total = 0;
    for (let i = 0; i < 50000000; i++) {
        total += step(array);
        if (allFlash(array)) {
            console.log("allFlash", i + 1);
            break;
        }
    }
    console.log(total);
}

function allFlash(array: number[][]): boolean {
    for (let i = 0; i < array.length; i++) {
        for (let j = 0; j < array[0].length; j++) {
            if (array[i][j] !== 0) return false;
        }
    }

    return true;
}

function step(array: number[][]): number {
    let flash = 0;
    for (let i = 0; i < array.length; i++) {
        for (let j = 0; j < array[0].length; j++) {
            array[i][j]++;
        }
    }

    const flashes = new Set<string>();
    let [queue, count] = flashAll(array, flashes);
    flash += count;
    while (queue.length) {
        for (let [p, q] of queue) {
            next(array, p, q, flashes);
        }
        const [newQueue, newCount] = flashAll(array, flashes);
        flash += newCount;
        queue = newQueue;
    }
    return flash;
}

function next(array: number[][], i: number, j: number, flashes: Set<string>) {
    const neighbors = getNeighbors(i, j, array);
    for (let [p, q] of neighbors) {
        if (!flashes.has(p + ":" + q)) {
            array[p][q]++;
        }
    }
}

function getNeighbors(i: number, j: number, array: number[][]): number[][] {
    const [m, n] = [array.length, array[0].length];
    return [
        [i - 1, j - 1],
        [i - 1, j],
        [i - 1, j + 1],
        [i, j - 1],
        [i, j + 1],
        [i + 1, j - 1],
        [i + 1, j],
        [i + 1, j + 1]
    ].filter(([x, y]) => x >= 0 && x < m && y >= 0 && y < n);
}

function flashAll(array: number[][], flashes: Set<string>): [[number, number][], number] {
    let count = 0;
    const queue: [number, number][] = [];
    for (let i = 0; i < array.length; i++) {
        for (let j = 0; j < array.length; j++) {
            if (array[i][j] >= 10) {
                queue.push([i, j]);
                flashes.add(i + ":" + j);
                array[i][j] = 0;
                count++;
            }
        }
    }
    return [queue, count];
}

solve();
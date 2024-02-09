import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2021/07/input");
    const splits = input.toString().split("\n");
    const positions = splits[0].split(",").map(d => parseInt(d));

    let minSum = Infinity;
    const maxPos = positions.reduce((a, b) => a > b ? a : b);
    for (let i = 0; i <= maxPos; i++) {
        let sum = 0;
        for (let position of positions) {
            const dist = Math.abs(position - i);
            sum += Math.round(dist * (dist + 1) / 2);
        }
        minSum = Math.min(sum, minSum);
    }
    console.log(minSum);
}

solve();
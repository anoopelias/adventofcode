import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("input");
    const splits = input.toString().split("\n").map(str => parseInt(str));

    let count = 0;
    let prev = splits[0] + splits[1] + splits[2];
    for (let i = 3; i < splits.length; i++) {
        const curr = prev - splits[i - 3] + splits[i];
        if (curr > prev) count++;
        prev = curr;
    }

    console.log(count);
}

console.log("Solving");
solve();

import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2022/01/input2");
    const splits = input.toString().split("\n");
    console.log("number of lines:", splits.length);
}

console.log("Solving");
solve();

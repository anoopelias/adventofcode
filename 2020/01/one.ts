import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/01/input.txt");
    const splits = input.toString().split("\n").map(str => parseInt(str));
    for (let i = 0; i < splits.length; i++) {
        for (let j = i; j < splits.length; j++) {
            if (splits[i] + splits[j] === 2020) {
                console.log(splits[i], splits[j], splits[i] * splits[j]);
            }

        }

    }
}

console.log("Solving");
solve();

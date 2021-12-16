import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("input.txt");
    const splits = input.toString().split("\n").map(str => parseInt(str));
    for (let i = 0; i < splits.length; i++) {
        for (let j = i; j < splits.length; j++) {
            for (let k = j; k < splits.length; k++) {
                if (splits[i] + splits[j] + splits[k] === 2020) {
                    console.log(splits[i] * splits[j] * splits[k]);
                }    
            }
        }

    }
}

console.log("Solving");
solve();

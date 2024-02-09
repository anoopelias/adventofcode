import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/06/input");
    const splits = input.toString().split("\n");
    const days = splits[0].split(",").map(d => parseInt(d));

    for (let i = 0; i < 256; i++) {
        let newCount = 0;

        for (let j = 0; j < days.length; j++) {
            if (days[j] === 0) {
                days[j] = 6;
                newCount++;
            } else {
                days[j]--;
            }
        }
        for (let k = 0; k < newCount; k++) {
            days.push(8);
        }
    }

    console.log(days.length);
}

solve();
import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2021/14/input");
    const splits = input.toString().split("\n");
    splits.pop();

    let pattern = splits[0];
    let i = 2;
    const map = new Map<string, string>();

    while (i < splits.length) {
        const [key, value] = splits[i].split(" -> ");
        map.set(key, value);
        i++;
    }

    for (let step = 0; step < 10; step++) {
        pattern = getOutput(pattern, map);
    }
    console.log("len", pattern.length);


    let freqs = new Array(26).fill(0);
    for (let i = 0; i < pattern.length; i++) {
        let charIndex = pattern.charCodeAt(i) - 65;
        freqs[charIndex]++;
    }

    freqs = freqs.filter(freq => freq !== 0);
    freqs.sort((a, b) => a - b);
    console.log(freqs[freqs.length - 1] - freqs[0]);
}

function getOutput(pattern: string, map: Map<string, string>) {
    let output = pattern.charAt(0);
    for (let i = 0; i < pattern.length - 1; i++) {
        const key = pattern.substring(i, i + 2);
        if (map.has(key)) {
            output += map.get(key)! + key.charAt(1);
        } else {
            output += key;
        }
    }
    return output;
}

solve();
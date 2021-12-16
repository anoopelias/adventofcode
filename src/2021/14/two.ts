import { promises as fs } from 'fs';

async function solve() {
    const splitLen = 200;
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

    let freqs = new Array(26).fill(0);
    let memo = new Map<string, number[]>();
    for (let i = 0; i < pattern.length - 1; i++) {
        addFreqs(freqs, getFreqs(pattern.substring(i, i + 2), map, 40, memo));
    }

    let charIndex = pattern.charCodeAt(pattern.length - 1) - 65;
    freqs[charIndex]++;

    freqs = freqs.filter(freq => freq !== 0);
    freqs.sort((a, b) => a - b);
    console.log(freqs[freqs.length - 1] - freqs[0]);

}

function getFreqs(pattern: string, map: Map<string, string>, steps: number, memo: Map<string, number[]>) {
    const key = pattern + ":" + steps;
    if (memo.has(key)) return memo.get(key)!;
    const freqs = new Array(26).fill(0);
    if (steps === 0 || !map.has(pattern)) {
        let charIndex = pattern.charCodeAt(0) - 65;
        freqs[charIndex]++;
        return freqs;
    }
    addFreqs(freqs, getFreqs(pattern.charAt(0) + map.get(pattern), map, steps - 1, memo));
    addFreqs(freqs, getFreqs(map.get(pattern) + pattern.charAt(1), map, steps - 1, memo));
    memo.set(key, freqs);
    return freqs;
}

function addFreqs(freq1: number[], freq2: number[]) {
    for (let i = 0; i < 26; i++) {
        freq1[i] += freq2[i];
    }
    return freq1;
}

solve();
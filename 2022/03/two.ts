import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2022/03/input");
    const splits = input.toString().split("\n");
    console.log("number of lines:", splits.length);

    let p = 0;

    for (let k = 0; k < splits.length / 3; k++) {
        const start = k * 3
        const first = splits[start];
        const sec = splits[start + 1]
        const third = splits[start + 2]

        for (let i = 0; i < first.length; i++) {
            const ch = first.charAt(i);
            if (sec.includes(ch) && third.includes(ch)) {
                p += valueOf(ch)
                break;
            }
        }
    }

    console.log(p);
}

function valueOf(ch: string): number {
    let v = 0;
    if (ch === ch.toUpperCase()) {
        v = ch.charCodeAt(0) - "A".charCodeAt(0) + 1 + 26
    } else {
        v = ch.charCodeAt(0) - "a".charCodeAt(0) + 1
    }
    return v;
}

console.log("Solving");
solve();

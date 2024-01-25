import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2022/03/input");
    const splits = input.toString().split("\n");
    console.log("number of lines:", splits.length);

    let p = 0;

    for (let k = 0; k < splits.length; k++) {
        const line = splits[k];
        const left = line.substring(0, line.length / 2)
        const right = line.substring(line.length / 2)

        for (let i = 0; i < left.length; i++) {
            if (right.includes(left.charAt(i))) {
                const ch = left.charAt(i);
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

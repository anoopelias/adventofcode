import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("input.txt");
    const splits = input.toString().split("\n");

    let valid = 0;
    for (let row of splits) {
        if (isValid(row)) valid++;
    }
    console.log(valid);
}

function isValid(row: string): boolean {
    const [policy, password] = row.split(":");
    const [minmax, letter] = policy.split(" ");
    const [min, max] = minmax.split("-").map(n => parseInt(n));

    let count = 0;
    for (let i = 0 ; i < password.length; i++) {
        if (password.charAt(i) === letter) {
            count++;
        }
    }

    return count >= min && count <= max;
}

console.log("Solving");
solve();

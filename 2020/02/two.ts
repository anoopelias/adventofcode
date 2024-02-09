import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/02/input.txt");
    const splits = input.toString().split("\n");

    let valid = 0;
    for (let row of splits) {
        if (isValid(row)) valid++;
    }
    console.log(valid);
}

function isValid(row: string): boolean {
    const [policy, password] = row.split(":").map(s => s.trim());
    const [minmax, letter] = policy.split(" ");
    const [pos1, pos2] = minmax.split("-").map(n => (parseInt(n) - 1));

    if (password.charAt(pos1) === letter && password.charAt(pos2) !== letter) {
        return true;
    }

    if (password.charAt(pos1) !== letter && password.charAt(pos2) === letter) {
        return true;
    }

    return false;
}

console.log("Solving");
solve();

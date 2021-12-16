import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("input");
    const splits = input.toString().split("\n");
    let i = 0;
    let j = 0;
    let aim = 0;

    for (let split of splits) {
        const [direction, length] = split.split(" ");

        if (direction === "forward") {
            i += parseInt(length);
            j += aim * parseInt(length);
        }
        else if (direction === "down") aim += parseInt(length);
        else if (direction === "up") aim -= parseInt(length);
    }

    console.log(i, j);
    console.log(i * j);
}

console.log("Solving");
solve();

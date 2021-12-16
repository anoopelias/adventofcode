import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2020/05/input");
    const splits = input.toString().split("\n");
    let max = 0;

    for (let split of splits) {
        let rowMin = 0;
        let rowMax = 127;
        for (let i = 0; i < 7; i++) {
            console.log("Before row", rowMin, rowMax);
            [rowMin, rowMax] = partition(split.charAt(i), rowMin, rowMax);
            console.log("After row", rowMin, rowMax);
        }

        let colMin = 0;
        let colMax = 7;
        for (let i = 7; i < 10; i++) {
            //console.log("Before", colMin, colMax);
            [colMin, colMax] = partition(split.charAt(i), colMin, colMax);
            //console.log("After", colMin, colMax);
        }

        if (rowMax !== rowMin || colMax !== colMin)
            console.log(rowMin, rowMax, colMin, colMax);
        const seat = (rowMin * 8) + colMin;
        max = Math.max(max, seat);
    }

    console.log(max);
}

function partition(letter: string, min: number, max: number) {
    const mid = min + Math.floor((max - min) / 2);
    if (letter === "F" || letter === "L") return [min, mid];
    return [mid + 1, max];
}

solve();
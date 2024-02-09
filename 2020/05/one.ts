import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/05/input");
    const splits = input.toString().split("\n");
    let max = 0;
    let seats: number[] = [];
    let minSeat = Number.MAX_SAFE_INTEGER;
    let maxSeat = 0;

    for (let split of splits) {
        let rowMin = 0;
        let rowMax = 127;
        for (let i = 0; i < 7; i++) {
            //console.log("Before row", rowMin, rowMax);
            [rowMin, rowMax] = partition(split.charAt(i), rowMin, rowMax);
            //console.log("After row", rowMin, rowMax);
        }

        let colMin = 0;
        let colMax = 7;
        for (let i = 7; i < 10; i++) {
            //console.log("Before", colMin, colMax);
            [colMin, colMax] = partition(split.charAt(i), colMin, colMax);
            //console.log("After", colMin, colMax);
        }

        const seat = (rowMin * 8) + colMin;
        max = Math.max(max, seat);
        minSeat = Math.min(minSeat, seat);
        maxSeat = Math.max(maxSeat, seat);

        seats.push(seat)
    }
    seats.sort((a, b) => a - b);
    console.log(seats)

    console.log(minSeat, maxSeat)
    for (let i = 1; i < seats.length; i++) {
        if (seats[i] != seats[i - 1] + 1) {
            console.log(seats[i - 1], seats[i], seats[i + 1])
        }
    }

    for (let i = minSeat; i <= maxSeat; i++) {
        if (!seats.includes(i)) {
            console.log(i)
        }
    }

}

function partition(letter: string, min: number, max: number) {
    const mid = min + Math.floor((max - min) / 2);
    if (letter === "F" || letter === "L") return [min, mid];
    return [mid + 1, max];
}

solve();
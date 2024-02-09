import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/06/input");
    const splits = input.toString().split("\n");
    const daysArr = splits[0].split(",").map(d => parseInt(d));
    const target = 256;
    const map = new Map<number, number>();

    let sum = 0;
    for (let days of daysArr) {
        sum += getCount(days, target, map);
    }

    //console.log(getCount(3, 80));
    console.log(sum);
}

function getCount(days: number, target: number, map: Map<number, number>): number {
    if (map.has(days)) return map.get(days)!;
    if (target - days <= 0) {
        return 1;
    }
    const sum = getCount(days + 7, target, map) + getCount(days + 9, target, map);
    map.set(days, sum);

    return sum;
}

solve();
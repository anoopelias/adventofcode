import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2022/01/input2");
    const splits = input.toString().split("\n");
    let hand = 0;
    let max = 0;
    const hands: number[] = [];


    for (let i =0; i < splits.length; i++) {
        if (splits[i].trim().length === 0) {
            hands.push(hand);
            max = Math.max(max, hand);
            hand = 0;
        } else {
            hand += parseInt(splits[i])
        }
    }
    hands.sort((a, b) => a - b);
    console.log(hands.pop()! + hands.pop()! + hands.pop()!)
    console.log(max)
}

console.log("Solving");
solve();

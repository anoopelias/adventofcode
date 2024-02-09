import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/03/input");
    const board = input.toString().split("\n").map(line => line.split(""));
    const [m, n] = [board.length, board[0].length];
    let i = 0;
    let j = 0;
    let trees = 0;
    console.log(m, n);

    while (i !== m - 1) {
        j += 3;
        j = j % n;
        i++;

        if (board[i][j] === '#') trees++;
    }
    console.log(trees);
}



console.log("Solving");
solve();

import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/03/input");
    const board = input.toString().split("\n").map(line => line.split(""));
    let trees = numberOfTrees(board, 1, 1);
    trees *= numberOfTrees(board, 3, 1);
    trees *= numberOfTrees(board, 5, 1);
    trees *= numberOfTrees(board, 7, 1);
    trees *= numberOf2Trees(board, 1, 2);

    console.log(trees);
}

function numberOfTrees(board: string[][], right: number, down: number): number {
    const [m, n] = [board.length, board[0].length];
    let i = 0;
    let j = 0;
    let trees = 0;
    console.log(m, n);

    while (i < m - 1) {
        j += right;
        j = j % n;
        i += down;
        if (board[i][j] === '#') trees++;
    }
    console.log(trees);
    return trees;
}

function numberOf2Trees(board: string[][], right: number, down: number): number {
    const [m, n] = [board.length, board[0].length];
    let i = 0;
    let j = 0;
    let trees = 0;
    console.log(m, n);

    while (i < m - 2) {
        j += right;
        j = j % n;
        i += down;
        if (board[i][j] === '#') trees++;
    }
    console.log(trees);
    return trees;
}

console.log("Solving");
solve();
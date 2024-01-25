import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2021/04/input");
    const splits = input.toString().split("\n");

    const draws = splits[0].split(",").map(s => parseInt(s));
    const boards: [number, boolean][][][] = [];
    let start = 1;

    while (start < splits.length) {
        start++;
        const board: [number, boolean][][] = [];
        for (let i = start; i < start + 5; i++) {
            board.push(splits[i]
                .split(" ")
                .filter(s => s !== "")
                .map(s => parseInt(s))
                .map(s => [s, false]));
        }
        start += 5;
        boards.push(board);
    }
    
    const winnerSet: Set<number> = new Set();
    for (let draw of draws) {
        for (let i = 0; i < boards.length; i++) {
            const board = boards[i];
            mark(board, draw);
            if (!winnerSet.has(i) && hasWinner(board)) {
                console.log("Winner", i, calcScore(board, draw));
                winnerSet.add(i);
            }
        }

    }
}

function calcScore(board: [number, boolean][][], finalDraw: number) {
    let sum = 0;
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[0].length; j++) {
            if (!board[i][j][1]) {
                sum += board[i][j][0];
            }
        }
    }
    return sum * finalDraw;
}

function hasWinner(board: [number, boolean][][]): boolean {
    for (let i = 0; i < board.length; i++) {
        let row = true;
        for (let j = 0; j < board[0].length; j++) {
            if (!board[i][j][1]) {
                row = false;
                break;
            }
        }

        if (row) return true;
    }

    for (let i = 0; i < board[0].length; i++) {
        let column = true;
        for (let j = 0; j < board.length; j++) {
            if (!board[j][i][1]) {
                column = false;
                break;
            }
        }

        if (column) return true;
    }

    return false;
}

function mark(board: [number, boolean][][], num: number) {
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[0].length; j++) {
            if (board[i][j][0] === num) {
                board[i][j][1] = true;
            }
        }
    }
}

console.log("Solving");
solve();
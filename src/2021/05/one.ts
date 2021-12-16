import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2021/05/input");
    const splits = input.toString().split("\n");

    const lines: [[number, number], [number, number]][] = [];
    let maxX = 0;
    let maxY = 0;

    for (let row of splits) {
        const [from, to] = row.split(" -> ");
        const [x1, y1] = from.split(",").map(str => parseInt(str));
        const [x2, y2] = to.split(",").map(str => parseInt(str));

        if (x1 == x2 || y1 == y2 || Math.abs(x1 - x2) === Math.abs(y1 - y2)) {
            lines.push([[x1, y1], [x2, y2]]);
            maxX = Math.max(maxX, x1, x2);
            maxY = Math.max(maxY, y1, y2);
        } 
    }

    const diagram: number[][] = [];

    for (let i = 0; i <= maxX; i++) {
        const row: number[] = [];
        diagram.push(row);
        for (let j = 0; j <= maxY; j++) {
            row.push(0);
        }
    }

    for (let line of lines) {
        const points = pointsOnLine(line[0], line[1]);
        for (let [x, y] of points) {
            diagram[x][y]++;
        }
    }

    let count = 0;
    for (let i = 0; i <= maxX; i++) {
        for (let j = 0; j <= maxY; j++) {
            if (diagram[i][j] >= 2) {
                count++;
            }
        }
    }
    console.log(count);
}

function pointsOnLine([x1, y1]: [number, number], [x2, y2]: [number, number]): [number, number][] {
    const ret: [number, number][] = [];

    if (x1 === x2) {
        const minY = Math.min(y1, y2);
        const maxY = (minY === y1)? y2: y1;

         for (let y = minY; y <= maxY; y++) {
             ret.push([x1, y]);
         }
    } else if (y1 === y2) {
        const minX = Math.min(x1, x2);
        const maxX = (minX === x1)? x2: x1;

        for (let x = minX; x <= maxX; x++) {
            ret.push([x, y1]);
        }
    } else if ((x1 < x2 && y1 < y2) ||
        (x1 > x2 && y1 > y2)) {
        const minX = Math.min(x1, x2);
        const maxX = minX === x1 ? x2: x1;
        let y = minX === x1 ? y1 : y2;

        for (let x = minX; x <= maxX; x++) {
            ret.push([x, y]);
            y++;
        }
    } else {
        const minX = Math.min(x1, x2);
        const maxX = minX === x1 ? x2: x1;
        let y = minX === x1 ? y1 : y2;
        for (let x = minX; x <= maxX; x++) {
            ret.push([x, y]);
            y--;
        }
    }

    return ret;
}
console.log("Solving");
solve();
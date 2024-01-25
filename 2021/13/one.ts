import { promises as fs } from 'fs';
import { addEmitHelper } from 'typescript';

async function solve() {
    const input = await fs.readFile("src/2021/13/input2");
    const splits = input.toString().split("\n");
    splits.pop();
    const dots: [number, number][] = [];
    let i = 0;

    while(splits[i].length !== 0) {
        const [x, y] = splits[i].split(",").map(s => parseInt(s));
        dots.push([x, y]);
        i++;
    }

    i++;
    const [strAxis, strVal] = splits[i].split("=");
    const value = parseInt(strVal);
    const axis = strAxis.charAt(strAxis.length - 1);

    const maxX = dots.reduce((maxX, [x, _]) => x > maxX? x: maxX, 0) + 1;
    const maxY = dots.reduce((maxY, [_, y]) => y > maxY? y: maxY, 0) + 1;

    const paper: boolean[][] = [];
    for (let i = 0; i < maxX; i++) {
        paper.push([]);
        for (let j = 0; j < maxY; j++) {
            paper[i].push(false);
        }
    }

    for (let [x, y] of dots) {
        paper[x][y] = true;
    }

    if (axis === "y") {
        let sourceY = value + 1;
        while (sourceY < maxY) {
            const targetY = value - (sourceY - value);
            for (let i = 0; i < maxX; i++) {
                if (paper[i][sourceY]) {
                    paper[i][targetY] = true;
                }
            }
            sourceY++;
        }

        let count = 0;
        for (let i = 0; i < maxX; i++) {
            for (let j = 0; j < value; j++) {
                if (paper[i][j]) count++;

            }
        }
        console.log(count);
    } else {
        let sourceX = value + 1;
        while (sourceX < maxX) {
            const targetX = value - (sourceX - value);
            for (let j = 0; j < maxY; j++) {
                if (paper[sourceX][j]) {
                    paper[targetX][i] = true;
                }
            }
            sourceX++;
        }

        let count = 0;
        for (let i = 0; i < value; i++) {
            for (let j = 0; j < maxY; j++) {
                if (paper[i][j]) count++;

            }
        }
        console.log(count);

    }


}

solve();
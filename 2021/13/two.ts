import { promises as fs } from 'fs';
import { addEmitHelper } from 'typescript';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2021/13/input");
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
    let folds: [string, number][] = [];
    while (i < splits.length) {
        const [strAxis, strVal] = splits[i].split("=");
        const value = parseInt(strVal);
        const axis = strAxis.charAt(strAxis.length - 1);
        folds.push([axis, value]);
        i++;
    
    }

    const maxX = dots.reduce((maxX, [x, _]) => x > maxX? x: maxX, 0) + 1;
    const maxY = dots.reduce((maxY, [_, y]) => y > maxY? y: maxY, 0) + 1;

    let paper: boolean[][] = [];
    for (let i = 0; i < maxY; i++) {
        paper.push([]);
        for (let j = 0; j < maxX; j++) {
            paper[i].push(false);
        }
    }

    for (let [x, y] of dots) {
        paper[y][x] = true;
    }

    for (let [axis, value] of folds) {
        paper = fold(paper, axis, value);
    }

    printPaper(paper);
}

function printPaper(paper: boolean[][]) {
    for (let i = 0; i < paper.length; i++) {
        console.log(paper[i].map(v => v? "|" : " ").join(""));
    }

}

function fold(paper: boolean[][], axis: string, value: number) {
    const [maxY, maxX] = [paper.length, paper[0].length];
    if (axis === "y") {
        let sourceY = value + 1;
        while (sourceY < maxY) {
            const targetY = value - (sourceY - value);
            for (let i = 0; i < maxX; i++) {
                if (paper[sourceY][i]) {
                    paper[targetY][i] = true;
                }
            }
            sourceY++;
        }
        return paper.slice(0, value);
    } else {
        let sourceX = value + 1;
        while (sourceX < maxX) {
            const targetX = value - (sourceX - value);
            for (let j = 0; j < maxY; j++) {
                if (paper[j][sourceX]) {
                    paper[j][targetX] = true;
                }
            }
            sourceX++;
        }
        return paper.map(row => row.slice(0, value));
    }

}

solve();
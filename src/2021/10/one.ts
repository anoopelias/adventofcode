import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2021/10/input");
    const splits = input.toString().split("\n");
    let scores: number[] = [];

    for (let line of splits) {
        const score = scoreFor(line);
        if (score !== null) {
            scores.push(score);
        }
    }

    scores.sort((a, b) => a - b);
    const mid = Math.floor(scores.length / 2);

    console.log(scores[mid]);
}

function scoreFor(line: string): number | null {
    const stack: string[] = [];

    for (let i = 0; i < line.length; i++) {
        const ch = line.charAt(i);
        if (ch === "(" || ch === "[" || ch === "{" || ch === "<") {
            stack.push(ch);
        } else {
            const tip = stack.pop();
            switch (ch) {
                case ")":
                    if (tip !== "(") return null;
                    break;
                case "]":
                    if (tip !== "[") return null;
                    break;
                case "}":
                    if (tip !== "{") return null;
                    break;
                case ">":
                    if (tip !== "<") return null;
                    break;
            }
        }
    }
    return points(stack);
}

function points(stack: string[]): number {

    let score = 0;

    while (stack.length) {
        score *= 5;
        const ch = stack.pop();

        switch (ch) {
            case "(":
                score += 1;
                break;
            case "[":
                score += 2;
                break;
            case "{":
                score += 3;
                break;
            case "<":
                score += 4;
                break;
        }


    }

    return score;

}
solve();
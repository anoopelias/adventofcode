import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2022/02/input");
    const splits = input.toString().split("\n");
    console.log("no of lines " + splits.length);
    let points = 0;

    for (let line of splits) {
        const letters = line.split(" ");
        points += winPoints(opponentValue(letters[0]), yourValue(letters[1])) +
            chosenPoints(yourValue(letters[1]));

    }
    console.log(points)
}

function winPoints(opponent: string, you: string) {
    if (opponent === "R") {
        if (you === "R") return 3
        else if (you === "P") return 6
        else return 0;
    } else if (opponent === "P") {
        if (you === "S") return 6
        else if (you === "P") return 3
        else return 0;
    } else if (opponent === "S") {
        if (you === "S") return 3
        else if (you === "R") return 6
        else return 0;
    }
    return 0;
}

function opponentValue(s: string) {
    if (s === "A")
        return "R";
    else if (s === "B")
        return "P";
    else return "S";
}

function yourValue(s: string) {
    if (s === "X")
        return "R";
    else if (s === "Y")
        return "P";
    else return "S";
}

function chosenPoints(s: string) {
    if (s == "R") {
        return 1;
    } else if (s == "P") {
        return 2;
    } else {
        return 3;
    }
}
console.log("Solving");
solve();

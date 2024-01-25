import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2022/02/input2");
    const splits = input.toString().split("\n");
    console.log("no of lines " + splits.length);
    let points = 0;

    for (let line of splits) {
        const letters = line.split(" ");
        points += winPoints(letters[1]) +
            chosenPoints(opponentValue(letters[0]), letters[1]);

    }
    console.log(points)
}

function chosenPoints(opponent: string, result: string) {
    if (opponent === "R") {
        if (result === "X") return chosenPointsVal("S")
        else if (result === "Y") return chosenPointsVal("R")
        else return chosenPointsVal("P");
    } else if (opponent === "P") {
        if (result === "X") return chosenPointsVal("R")
        else if (result === "Y") return chosenPointsVal("P")
        else return chosenPointsVal("S");
    } else if (opponent === "S") {
        if (result === "X") return chosenPointsVal("P")
        else if (result === "Y") return chosenPointsVal("S")
        else return chosenPointsVal("R");
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

function winPoints(s: string) {
    if (s == "X") {
        return 0;
    } else if (s == "Y") {
        return 3;
    } else {
        return 6;
    }
}

function chosenPointsVal(s: string) {
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

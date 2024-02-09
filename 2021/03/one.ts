import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2021/03/input");
    const splits = input.toString().split("\n");

    let oxygens = splits.slice();
    let i = 0;
    while (oxygens.length > 1) {
        const [zeroCount, oneCount] = getCountList(oxygens, i);
        oxygens = oxygens.filter(row => {
            if (zeroCount > oneCount) {
                return row.charAt(i) === "0";
            } else {
                return row.charAt(i) === "1";
            }
        });
        i++;
    }

    let co2 = splits.slice();
    i = 0;
    while (co2.length > 1) {
        const [zeroCount, oneCount] = getCountList(co2, i);
        co2 = co2.filter(row => {
            if (oneCount < zeroCount) {
                return row.charAt(i) === "1";
            } else {
                return row.charAt(i) === "0";
            }
        });
        i++;
    }

    console.log(oxygens, co2);
    console.log(parseInt(oxygens[0], 2) * parseInt(co2[0], 2));
}

function getCountList(splits: string[], i: number): [number, number] {
    let zeroCount = 0;
    let oneCount = 0;

    for (let row of splits) {
        if (row.charAt(i) === "0") {
            zeroCount++;
        } else {
            oneCount++;
        }
    }

    return [zeroCount, oneCount];
}

console.log("Solving");
solve();
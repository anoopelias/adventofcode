import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2020/04/input");
    const splits = input.toString().split("\n");

    const passports: Map<string, string>[] = [];
    let currMap = new Map<string, string>();
    for (let i = 0; i < splits.length; i++) {
        const keyValues = splits[i].split(" ");
        if (keyValues.length === 1 && keyValues[0] === "") {
            passports.push(currMap);
            currMap = new Map<string, string>();
        } else {
            for (let keyValue of keyValues) {
                const [key, value] = keyValue.split(":");
                if (isValid(key, value)) {
                    currMap.set(key, value);
                }
            }
        }
    }

    let valid = 0;
    for (let passport of passports) {
        if (passport.has("byr") &&
            passport.has("iyr") &&
            passport.has("eyr") &&
            passport.has("hgt") &&
            passport.has("hcl") &&
            passport.has("ecl") &&
            passport.has("pid")) {
            valid++;
        }
    }
    console.log(valid);
}

function isValid(key: string, value: string): boolean {

    if (key === "byr") {
        if (!isMatch(value, /^([0-9]{4})$/, 1920, 2002)) return false;
    }

    if (key === "iyr") {
        if (!isMatch(value, /^([0-9]{4})$/, 2010, 2020)) return false;
    }

    if (key === "eyr") {
        if (!isMatch(value, /^([0-9]{4})$/, 2020, 2030)) return false;
    }

    if (key === "hgt") {
        const regex = /^(\d+)(in|cm)$/;
        const match = value.match(regex);
        if (!match) return false;

        if (match.length !== 3) return false;
        if (match[2] === "in") {
            const height = parseInt(match[1]);
            if (height < 59 || height > 76) return false;
        } else if (match[2] === "cm") {
            const height = parseInt(match[1]);
            if (height < 150 || height > 193) return false;
        }
    }
    if (key === "hcl") {
        const regex = /^#[0-9a-f]{6}$/g;
        if (!value.match(regex)) return false;
    }
    if (key === "ecl") {
        const regex = /^(amb|blu|brn|gry|grn|hzl|oth)$/g;
        if (!value.match(regex)) return false;
    }
    if (key === "pid") {
        const regex = /^\d{9}$/g;
        if (!value.match(regex)) return false;
    }

    return true;
}

function isMatch(value: string, regex: RegExp, min: number, max: number): boolean {
    const match = value.match(regex);
    if (!match) return false;
    const num = parseInt(match[1]);
    if (num < min || num > max) return false;

    return true;
}

console.log("Solving");
solve();

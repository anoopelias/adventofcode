import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2021/08/input");
    const splits = input.toString().split("\n");
    let sum = 0;

    for (let split of splits) {
        const inpOut = split.split(" | ");
        const input = inpOut[0].split(" ");
        const output = inpOut[1].split(" ");

        const one = input.filter(num => num.length === 2)[0];
        const four = input.filter(num => num.length === 4)[0];
        const seven = input.filter(num => num.length === 3)[0];
        const eight = input.filter(num => num.length === 7)[0];
        const zeroSixNine = input.filter(num => num.length === 6);
        const twoThreeFive = input.filter(num => num.length === 5);
        const six = zeroSixNine.filter(num => !contains(num, one))[0];
        const c = exclude(one, six);
        const f = exclude(one, c);
        
        const two = twoThreeFive.filter(num => !contains(num, f))[0];
        const five = twoThreeFive.filter(num => num !== two && contains(num, f))[0];
        const three = twoThreeFive.filter(num => num !== two && num !== five)[0];

        const a = exclude(seven, one);
        const b = exclude(eight, two + f);
        const d = exclude(four, b + c + f);
        const g = exclude(three, a + c + d + f);
        const e = exclude(two, a + c + d + g);

        let numVal = 0;
        let exponent = 1000;
        for (let num of output) {
            let value = -1;
            if (num.length === 2) value = 1;
            else if (num.length === 4) value = 4;
            else if (num.length === 3) value = 7;
            else if (num.length === 7) value = 8;
            else if (num.length === 6) {
                if (num.indexOf(d) === -1) value = 0;
                else if (num.indexOf(e) === -1) value = 9;
                else value = 6;
            } else if (num.length === 5) {
                if (num.indexOf(b) === -1 && num.indexOf(f) === -1) value = 2;
                else if (num.indexOf(b) === -1 && num.indexOf(e) === -1) value = 3;
                else value = 5;
            }
            numVal += value * exponent;
            exponent = exponent / 10;
        }
        sum += numVal;
    }

    console.log(sum);
}

function contains(str1: string, str2: string): boolean {

    for (let i = 0; i < str2.length; i++) {
        if (str1.indexOf(str2.charAt(i)) === -1) {
            return false;
        }
    }

    return true;
}

// Find characters in str1 but not in str2
function exclude(str1: string, str2: string): string {
    let ret = "";
    for (let i = 0; i < str1.length; i++) {
        if (str2.indexOf(str1.charAt(i)) === -1) {
            ret += str1.charAt(i);
        }
    }

    return ret;
}
/*

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
8       2|3|5 2|3|5 2|3|5 7   0|9    6      4    0|9    1
                          acf               bcdf        cf 
                      
 aaaa1   
b    c
b    c1
 dddd
e    f1
e    f
 gggg

0 - 6
1 - 2
2 - 5
3 - 5
4 - 4
5 - 5
6 - 6
7 - 3
8 - 7
9 - 6

*/
solve();
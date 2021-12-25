import { promises as fs } from 'fs';

async function solve() {
    const input = await fs.readFile("src/2021/24/input_3_14");
    //const input = await fs.readFile("src/2021/24/input3");
    const splits = input.toString().split("\n");
    splits.pop();

    processAlu(splits, [9, 9, 9, 6, 9, 7, 9, 9, 7, 9, 9, 5], 0);
}

function processAlu(ins: string[], input: number[], zstart: number) {
    const alu = new ALU(input);
    alu.z = zstart;
    for (let split of ins) {
        const ins = split.split(" ");
        switch (ins[0]) {
            case "inp":
                alu.processInp(ins);
                break;
            case "add":
                alu.processAdd(ins);
                break;
            case "mul":
                alu.processMul(ins);
                break;
            case "div":
                alu.processDiv(ins);
                break;
            case "mod":
                alu.processMod(ins);
                break;
            case "eql":
                alu.processEqual(ins);
                break;
        }
    }
    console.log([alu.w, alu.x, alu.y, alu.z].join("\t"));
    if (alu.z === 0) {
        console.log(input, zstart);
    }
}

class ALU {
    w: number;
    x: number;
    y: number;
    z: number;
    inputs: number[];
    inputIndex = 0;

    constructor(inputs: number[]) {
        this.w = 0;
        this.x = 0;
        this.y = 0;
        this.z = 24;
        this.inputs = inputs;
    }

    nextInput(): number {
        return this.inputs[this.inputIndex++];
    }


    processInp(args: string[]) {
        switch (args[1]) {
            case "w":
                this.w = this.nextInput();
                break;
            case "x":
                this.x = this.nextInput();
                break;
            case "y":
                this.y = this.nextInput();
                break;
            case "z":
                this.z = this.nextInput();
                break;
        }
    }

    valueOf(ch: string): number {
        switch (ch) {
            case "w":
                return this.w;
                break;
            case "x":
                return this.x;
                break;
            case "y":
                return this.y;
                break;
            case "z":
                return this.z;
                break;

        }
        return -1;
    }

    processAdd(args: string[]) {
        let arg2 = parseInt(args[2]);
        if (isNaN(arg2)) {
            arg2 = this.valueOf(args[2]);
        }
        switch (args[1]) {
            case "w":
                this.w += arg2;
                break;
            case "x":
                this.x += arg2;
                break;
            case "y":
                this.y += arg2;
                break;
            case "z":
                this.z += arg2;
                break;
        }
    }
    processMul(args: string[]) {
        let arg2 = parseInt(args[2]);
        if (isNaN(arg2)) {
            arg2 = this.valueOf(args[2]);
        }
        switch (args[1]) {
            case "w":
                this.w *= arg2;
                break;
            case "x":
                this.x *= arg2;
                break;
            case "y":
                this.y *= arg2;
                break;
            case "z":
                this.z *= arg2;
                break;
        }
    }
    processDiv(args: string[]) {
        let arg2 = parseInt(args[2]);
        if (isNaN(arg2)) {
            arg2 = this.valueOf(args[2]);
        }
        switch (args[1]) {
            case "w":
                this.w = Math.floor(this.w / arg2);
                break;
            case "x":
                this.x = Math.floor(this.x / arg2);
                break;
            case "y":
                this.y = Math.floor(this.y / arg2);
                break;
            case "z":
                this.z = Math.floor(this.z / arg2);
                break;
        }
    }
    processMod(args: string[]) {
        let arg2 = parseInt(args[2]);
        if (isNaN(arg2)) {
            arg2 = this.valueOf(args[2]);
        }
        switch (args[1]) {
            case "w":
                this.w %= arg2;
                break;
            case "x":
                this.x %= arg2;
                break;
            case "y":
                this.y %= arg2;
                break;
            case "z":
                this.z %= arg2;
                break;
        }
    }
    processEqual(args: string[]) {
        let arg2 = parseInt(args[2]);
        if (isNaN(arg2)) {
            arg2 = this.valueOf(args[2]);
        }
        switch (args[1]) {
            case "w":
                this.w = (this.w === arg2) ? 1 : 0;
                break;
            case "x":
                this.x = (this.x === arg2) ? 1 : 0;
                break;
            case "y":
                this.y = (this.y === arg2) ? 1 : 0;
                break;
            case "z":
                this.z = (this.z === arg2) ? 1 : 0;
                break;
        }
    }

}
solve();
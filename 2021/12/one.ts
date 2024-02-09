import { promises as fs } from 'fs';
import { addEmitHelper } from 'typescript';

async function solve() {
    const input = await fs.readFile("../../../aoc-files/2021/12/input");
    const splits = input.toString().split("\n");
    splits.pop();

    const adj: Map<string, string[]> = new Map<string, string[]>();
    for (let split of splits) {
        const [from, to] = split.split("-");
        if (!adj.has(from)) {
            adj.set(from, []);
        }
        if (!adj.has(to)) {
            adj.set(to, []);
        }
        adj.get(from)!.push(to);
        adj.get(to)!.push(from);
    }

    const result = noOfPaths(adj, "start", new Set<string>(), false);
    console.log(result);
}

function noOfPaths(adj: Map<string, string[]>, from: string, elems: Set<string>, small: boolean): number {
    const [targets, smallRepeats] = getTargets(adj, from, elems);
    let count = 0;

    for (let target of targets) {
        if (target === "end") count++;
        else {
            elems.add(target);
            count += noOfPaths(adj, target, elems, small);
            elems.delete(target);
        }
    }
    if (!small) {
        for (let smallRepeat of smallRepeats) {
            count += noOfPaths(adj, smallRepeat, elems, true);
        }
    }
    return count;
}

function getTargets(adj: Map<string, string[]>, from: string, elems = new Set<string>()) {
    const tos = adj.get(from)!;
    const targets: string[] = [];
    const smallRepeats: string[] = [];

    for (let to of tos) {
        if (to === "start") continue;
        if (to === "end") {
            targets.push(to);
        } else if (to.charCodeAt(0) < 97) {
            targets.push(to);
        } else {
            if (!elems.has(to)) {
                targets.push(to);
            } else {
                smallRepeats.push(to);
            }
        }
    }

    return [targets, smallRepeats];
}


solve();
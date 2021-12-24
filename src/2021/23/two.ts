
function solve() {
    //const startState = new State([["A", "D", "D", "B"], ["D", "B", "C", "C"], ["C", "A", "B", "B"], ["A", "C", "A", "D"]]);
    const startState = new State([["D", "D", "D", "D"], ["C", "B", "C", "A"], ["B", "A", "B", "C"], ["B", "C", "A", "A"]]);
    const endState = new State([["A", "A", "A", "A"], ["B", "B", "B", "B"], ["C", "C", "C", "C"], ["D", "D", "D", "D"]]);

    startState.print();

    const distMap = new Map<string, [number, State | null]>();
    distMap.set(startState.getHash(), [0, null]);
    const minPQ = new MinPQ<State>(startState);

    while (!minPQ.isEmpty()) {
        const [_, state] = minPQ.delMin();
        const dist = distMap.get(state.getHash())![0];
        if (state.getHash() === endState.getHash()) {
            printPath(distMap, state, dist);
            break;
        }
        for (let [cost, nextState] of state.getNextStates()) {
            const newDist = dist + cost;
            const hash = nextState.getHash();
            if (distMap.has(hash) && distMap.get(hash)![0] <= newDist) {
                continue;
            }
            distMap.set(hash, [newDist, state]);
            minPQ.insert(newDist, nextState);
        }
    }
}

function printPath(distMap: Map<string, [number, State | null]>, state: State, dist: number) {
    let routeState: State | null = state;
    while (routeState !== null) {
        console.log(dist);
        routeState.print();
        [dist, routeState] = distMap.get(routeState.getHash())!;
    }
}

const roomNumPos = [
    2,
    4,
    6,
    8
]

const roomNums: { [key: string]: number } = {
    "A": 0,
    "B": 1,
    "C": 2,
    "D": 3
}

const stepCost: { [key: string]: number } = {
    "A": 1,
    "B": 10,
    "C": 100,
    "D": 1000,
}

const roomToHallway: [number, number[]][][] = [
    [
        [0, [1]],
        [1, []],
        [3, []],
        [5, [3]],
        [7, [3, 5]],
        [9, [3, 5, 7]],
        [10, [3, 5, 7, 9]],
    ],
    [
        [0, [1, 3]],
        [1, [3]],
        [3, []],
        [5, []],
        [7, [5]],
        [9, [5, 7]],
        [10, [5, 7, 9]],
    ],
    [
        [0, [1, 3, 5]],
        [1, [3, 5]],
        [3, [5]],
        [5, []],
        [7, []],
        [9, [7]],
        [10, [7, 9]],
    ],
    [
        [0, [1, 3, 5, 7]],
        [1, [3, 5, 7]],
        [3, [5, 7]],
        [5, [7]],
        [7, []],
        [9, []],
        [10, [9]],
    ]
];

function roomStr(room: string[], i: number): string {
    return room[3 - i] ? "." : room[3 - i];
}

class State {
    hallway: string[];
    rooms: string[][];

    constructor(rooms: string[][], hallway?: string[]) {
        if (hallway) {
            this.hallway = hallway;
        } else {
            this.hallway = new Array(11).fill(".");
        }
        this.rooms = [];
        this.rooms = rooms;
    }

    print() {
        console.log("#############");
        console.log("#" + this.hallway.join("") + "#");
        console.log("###" + this.rooms.map(room => roomStr(room, 0)).join("#") + "###");
        console.log("  #" + this.rooms.map(room => roomStr(room, 1)).join("#") + "#");
        console.log("  #" + this.rooms.map(room => roomStr(room, 2)).join("#") + "#");
        console.log("  #" + this.rooms.map(room => roomStr(room, 3)).join("#") + "#");
        console.log("  #########");
        console.log("");
    }

    getHash() {
        return this.hallway.join(",") + ":" +
            this.rooms.map(room => room.join(",")).join(":");
    }

    newRooms(roomNum: number, letter?: string): string[][] {
        const rooms = this.rooms.map(room => room.slice());
        if (letter) {
            rooms[roomNum].push(letter);
        } else {
            rooms[roomNum].pop();
        }
        return rooms;
    }

    newHallway(position: number, letter?: string) {
        const hallway = this.hallway.slice();
        hallway[position] = letter ? letter : ".";
        return hallway;
    }

    roomOutSteps(roomNo: number): number {
        return 5 - this.rooms[roomNo].length;
    }

    roomInSteps(roomNo: number): number {
        return 4 - this.rooms[roomNo].length;
    }

    isEmpty(positions: number[]): boolean {
        return positions.map(i => this.hallway[i]).every(s => s === ".");
    }

    isPathEmpty(pos1: number, pos2: number): boolean {
        const minPos = Math.min(pos1, pos2);
        const maxPos = Math.max(pos1, pos2);

        const positions: number[] = [];

        for (let i = minPos; i <= maxPos; i++) {
            positions.push(i);
        }

        return this.isEmpty(positions);
    }

    getNextStatesFromRoom(roomNo: number): [number, State][] {
        const nextStates: [number, State][] = [];
        const room = this.rooms[roomNo];
        if (room.length === 0) return nextStates;
        const letter = room[room.length - 1];
        const newRoomNo = roomNums[letter];

        if (newRoomNo !== roomNo && this.canGoToRoom(letter)
            && this.isPathEmpty(roomNumPos[newRoomNo], roomNumPos[roomNo])) {
            // room to room move

            const steps = this.roomOutSteps(roomNo) +
                (2 * Math.abs(newRoomNo - roomNo)) +
                this.roomInSteps(newRoomNo);
            const cost = stepCost[letter] * steps;

            const newRooms = this.newRooms(newRoomNo, letter);
            newRooms[roomNo].pop();
            const newHallway = this.hallway.slice();
            nextStates.push([cost, new State(newRooms, newHallway)]);
        }

        for (let hallwayPath of roomToHallway[roomNo]) {
            const [pos, path] = hallwayPath;
            if (this.isEmpty([...path, pos])) {
                // room to hallway
                const steps = this.roomOutSteps(roomNo) + Math.abs(pos - roomNumPos[roomNo]);
                const cost = steps * stepCost[letter];
                const newRooms = this.newRooms(roomNo);
                const newHallway = this.newHallway(pos, letter);
                nextStates.push([cost, new State(newRooms, newHallway)]);
            }
        }

        return nextStates;
    }

    getNextStatesFromHallway(): [number, State][] {
        const nextStates: [number, State][] = [];
        for (let i = 0; i < this.hallway.length; i++) {
            let letter = this.hallway[i];
            if (letter !== ".") {
                let roomNo = roomNums[letter];
                let roomPos = roomNumPos[roomNo];
                if (this.canGoToRoom(letter) && this.isPathEmpty(roomPos, roomPos > i ? i + 1 : i - 1)) {
                    let steps = Math.abs(i - roomNumPos[roomNo]) + this.roomInSteps(roomNo);
                    let cost = steps * stepCost[letter];
                    let newRooms = this.newRooms(roomNums[letter], letter);
                    let newHallway = this.newHallway(i);
                    nextStates.push([cost, new State(newRooms, newHallway)])
                }
            }
        }
        return nextStates;
    }

    getNextStates(): [number, State][] {
        let nextStates = this.getNextStatesFromHallway();
        for (let i = 0; i < this.rooms.length; i++) {
            nextStates = nextStates.concat(this.getNextStatesFromRoom(i));
        }
        return nextStates;
    }

    canGoToRoom(letter: string) {
        const num = roomNums[letter];
        return (this.rooms[num].length === 0 ||
            (this.rooms[num].length <= 3 && this.rooms[num].every(s => s === letter)));
    }

}


class MinPQ<T> {
    private arr: [number, T][] = [];

    constructor(t: T, distance = 0) {
        this.arr.push([-1, t]);
        this.insert(distance, t);
    }

    insert(n: number, t: T) {
        this.arr.push([n, t]);
        this.swim(this.arr.length - 1);
    }

    delMin(): [number, T] {
        const max = this.arr[1];
        const newVal = this.arr.pop();

        if (!this.isEmpty()) {
            this.arr[1] = newVal!;
            this.sink(1);
        }

        return max;
    }

    isEmpty(): boolean {
        return this.arr.length === 1;
    }

    min(): [number, T] {
        return this.arr[1];
    }

    size(): number {
        return this.arr.length - 1;
    }

    private swim(n: number) {
        if (n === 1) return;
        const parent = Math.floor(n / 2);

        const arr = this.arr;
        if (arr[n][0] < arr[parent][0]) {
            [arr[n], arr[parent]] = [arr[parent], arr[n]];
            this.swim(parent);
        }
    }

    private sink(n: number) {
        const arr = this.arr;
        const left = n * 2;
        const right = left + 1;

        if (left >= arr.length) return;

        const leaf = (right < arr.length && arr[right][0] < arr[left][0]) ? right : left;

        if (arr[n][0] <= arr[leaf][0]) return;

        [arr[n], arr[leaf]] = [arr[leaf], arr[n]];
        this.sink(leaf);
    }
}

solve();
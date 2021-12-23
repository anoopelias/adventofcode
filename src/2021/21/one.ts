import { promises as fs } from 'fs';

async function solve() {
  // const input = await fs.readFile("src/2021/20/input");
  // const splits = input.toString().split("\n");
  // splits.pop();

  let pos0 = 8;
  let score0 = 0;
  let pos1 = 4;
  let score1 = 0;

  let dice = new Dice();
  let zero = true;
  let count = 0;
  while (score0 < 1000 && score1 < 1000) {
    if (zero) {
      pos0 += dice.roll() + dice.roll() + dice.roll();
      pos0 = ((pos0 - 1) % 10) + 1;
      score0 += pos0;
      count += 3;

      zero = false;
    } else {
      pos1 += dice.roll() + dice.roll() + dice.roll();
      pos1 = ((pos1 - 1) % 10) + 1;
      score1 += pos1;
      count += 3;

      zero = true;
    }
  }
  console.log(score0, score1, count);
}

class Universe {
  player0: number;
  player1: number;
  dice: Dice;

  constructor(player0: number, player1: number, dice: number) {
    this.player0 = player0;
    this.player1 = player1;
    this.dice = new Dice()
  }

  play() {

  }
}

class Dice {
  value = 0;

  roll() {
    this.value++;
    if (this.value === 101) {
      this.value = 1;
    }
    return this.value;
  }
}

solve();

/*
1 1 1 - 3 4 5
        4 5 6
        5 6 7
1 2 1 - 4 5 6
        5 6 7
        6 7 8
1 3 1 - 5 6 7
        6 7 8
        7 8 9

3 - 1
4 - 2
5 - 5
6 - 7
7 - 6
8 - 3
9 - 1



*/
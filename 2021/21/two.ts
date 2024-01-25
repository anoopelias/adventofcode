import { promises as fs } from 'fs';
import { rootCertificates } from 'tls';

async function solve() {
  let pos0 = 4;
  let score0 = 0;
  let pos1 = 8;
  let score1 = 0;

  const [zeroWins, oneWins] = wins(score0, score1, pos0, pos1, false, 0, 0, new Map<string, [bigint, bigint]>());
  console.log(zeroWins, oneWins);
}

function wins(score0: number, score1: number, pos0: number, pos1: number, zero: boolean,
  rolls: number, dices: number, memo: Map<string, [bigint, bigint]>): [bigint, bigint] {

  if (score0 >= 21) return [BigInt(1), BigInt(0)];
  if (score1 >= 21) return [BigInt(0), BigInt(1)];
  const key = score0 + ":" + score1 + ":" + pos0 + ":" + pos1 + ":" + zero + ":" + rolls + ":" + dices;
  if (memo.has(key)) {
    return memo.get(key)!;
  }

  let [zeroWins, oneWins] = [BigInt(0), BigInt(0)];
  if (rolls !== 3) {
    for (let i = 1; i <= 3; i++) {
      let [currZeroWins, currOneWins] = wins(score0, score1, pos0, pos1, zero, rolls + 1, dices + i, memo);
      zeroWins = currZeroWins + zeroWins;
      oneWins = currOneWins + oneWins;
    }
    memo.set(key, [zeroWins, oneWins]);
    return [zeroWins, oneWins];
  }

  if (zero) {
    const [currScore0, currPos0] = next(score0, pos0, dices);
    return wins(currScore0, score1, currPos0, pos1, !zero, 0, 0, memo);
  } else {
    const [currScore1, currPos1] = next(score1, pos1, dices);
    return wins(score0, currScore1, pos0, currPos1, !zero, 0, 0, memo);
  }
}

function next(score: number, pos: number, dice: number): [number, number] {
  pos += dice;
  pos = ((pos - 1) % 10) + 1;
  score += pos;
  return [score, pos];
}


solve();
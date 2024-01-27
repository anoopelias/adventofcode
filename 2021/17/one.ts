import { X25519KeyPairKeyObjectOptions } from 'crypto';
import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("src/2021/17/input");
  let splits = input.toString().split("\n");
  splits.pop();

  splits = splits[0].split("=");
  const [x1, x2] = splits[1].substring(0, splits[1].indexOf(",")).split("..").map(s => parseInt(s));
  const [y1, y2] = splits[2].split("..").map(s => parseInt(s));
  let maxY = 0;
  let count = 0;
  let range = -3;
  for (let vx = 0; vx < 1000; vx++) {
    for (let vy = -500; vy < 500; vy++) {
      let [lvx, lvy] = [vx, vy];
      let [x, y] = [0, 0];
      let lMaxY = 0;

      while(true) {
        [[x, y], [lvx, lvy]] = nextStep([x, y], [lvx, lvy]);
        lMaxY = Math.max(lMaxY, y);
        range = isInRange([x, y], [x1, y1], [x2, y2])
        if (range === 0) {
          maxY = Math.max(lMaxY, maxY);
          count++;
          break;
        } else if (range < 0) {
          break;
        }
      }
    }
  }
  console.log(count);
  console.log(maxY);
}

function isInRange([x, y]: [number, number], [x1, y1]: [number, number], [x2, y2]: [number, number]) {
  if (x >= x1 && x <= x2 && y >= y1 && y <= y2) {
    return 0;
  }

  if (x > x2) {
    return -1;
  }

  if (y < y1) {
    return -2;
  }

  return 1;
}

function nextStep([x, y]: [number, number], [vx, vy]: [number, number]) {
  return [
    [x + vx, y + vy],
    [vx > 0 ? vx - 1 : vx === 0 ? vx : vx + 1, vy - 1],
  ]
}

solve();
import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("../../../aoc-files/2021/20/input");
  const splits = input.toString().split("\n");
  splits.pop();

  const algo = splits[0];
  let image: string[][] = [];
  
  for (let i = 2; i < splits.length; i++) {
    image.push(splits[i].split(""));
  }
  pad(image, ".");

  for (let i = 0; i<50; i++) {
    image = nextStep(algo, image);
  }

  let count = 0;
  let [m, n] = [image.length, image[0].length];
  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      if (image[i][j] === "#") count++;
    }
  }

  console.log(count);
}

function printImage(image: string[][]) {
  console.log(image.map(row => row.join("")).join("\n"));
  console.log("");
}

function nextStep(algo: string, image: string[][]) {
  let [m, n] = [image.length, image[0].length];
  const newImage: string[][] = [];
  for (let i = 0; i < m; i++) {
    const row: string[] = [];
    for (let j = 0; j < n; j++) {
      if (i === 0 || i === m - 1 || j === 0 || j === n - 1) {
        if (algo.charAt(0) === "#") {
          row.push(image[i][j] === "#"? "." : "#")
        }
      } else {
        const index = getIndex(image, [i, j]);
        row.push(algo.charAt(index));
      }
    }
    newImage.push(row);
  }
  pad(newImage, newImage[0][0]);
  return newImage;
}

function getIndex(image: string[][], [i, j]: [number, number]): number {
  const binary = getNeighbors([i, j]).map(([p, q]) => image[p][q] === "#"? "1" : "0").join("");
  return parseInt(binary, 2);
}

function getNeighbors([i, j]: [number, number]): [number, number][] {
  return [
      [i - 1, j - 1],
      [i - 1, j],
      [i - 1, j + 1],
      [i, j - 1],
      [i, j],
      [i, j + 1],
      [i + 1, j - 1],
      [i + 1, j],
      [i + 1, j + 1]
  ];
}

function pad(image: string[][], ch: string) {
  let [m, n] = [image.length, image[0].length];
  let och = ch === "#"? "." : "#";
  let topPad = 0;
  if (hasRow(image, 0, och)) topPad = 2;
  else if (hasRow(image, 1, och)) topPad = 1;

  let bottomPad = 0;
  if (hasRow(image, m - 1, och)) bottomPad = 2;
  else if (hasRow(image, m - 2, och)) bottomPad = 1;

  let leftPad = 0;
  if (hasColumn(image, 0, och)) leftPad = 2;
  else if (hasColumn(image, 1, och)) leftPad = 1;

  let rightPad = 0;
  if (hasColumn(image, n - 1, och)) rightPad = 2;
  else if (hasColumn(image, n - 2, och)) rightPad = 1;

  for (let i = 0; i < topPad; i++) {
    image.unshift(new Array(n).fill(ch));
  }

  for (let i = 0; i < bottomPad; i++) {
    image.push(new Array(n).fill(ch));
  }
  m = m + topPad + bottomPad;

  for (let i = 0; i < leftPad; i++) {
    for (let j = 0; j < m; j++) {
      image[j].unshift(ch);
    }
  }

  for (let i = 0; i < rightPad; i++) {
    for (let j = 0; j < m; j++) {
      image[j].push(ch);
    }
  }
}

function hasColumn(image: string[][], col: number, ch: string) {
  let [m, _] = [image.length, image[0].length];
  for (let p = 0; p < m; p++) {
    if (image[p][col] === ch) return true;
  }
  return false;
}


function hasRow(image: string[][], row: number, ch: string) {
  let [_, n] = [image.length, image[0].length];
  for (let p = 0; p < n; p++) {
    if (image[row][p] === ch) return true;
  }
  return false;
}

solve();
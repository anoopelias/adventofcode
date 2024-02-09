import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("../../../aoc-files/2021/16/input");
  const splits = input.toString().split("\n");
  splits.pop();
  const packets: Packet[] = [];
  const str = splits[0];
  const [_, packet] = readPacket(0, str);
  //printPacket(packet);
  console.log(sumOfVersions(packet));
  console.log(valueOf(packet));
}

function sumOfVersions(packet: Packet) {
  let sum = packet.version;
  for (let subPacket of packet.subPackets) {
    sum += sumOfVersions(subPacket);
  }
  return sum;
}

function valueOf(packet: Packet): bigint {
  let value = BigInt(-1);
  if (packet.type === 4) {
    value = packet.value!;
  }
  if (packet.type === 0) {
    value = BigInt(0);
    for (let subPacket of packet.subPackets) {
      value += valueOf(subPacket);
    }
  }
  if (packet.type === 1) {
    value = BigInt(1);
    for (let subPacket of packet.subPackets) {
      const newValue = valueOf(subPacket);
      value = value * newValue;
    }
  }
  if (packet.type === 2) {
    value = valueOf(packet.subPackets[0]);
    for (let subPacket of packet.subPackets) {
      const newValue = valueOf(subPacket);
      value = newValue < value ? newValue : value;
    }
  }
  if (packet.type === 3) {
    value = valueOf(packet.subPackets[0]);
    for (let subPacket of packet.subPackets) {
      const newValue = valueOf(subPacket);
      value = newValue > value ? newValue : value;
    }
  }
  if (packet.type === 5) {
    value = valueOf(packet.subPackets[0]) > valueOf(packet.subPackets[1]) ? BigInt(1) : BigInt(0);
  }
  if (packet.type === 6) {
    value = valueOf(packet.subPackets[0]) < valueOf(packet.subPackets[1]) ? BigInt(1) : BigInt(0);
  }
  if (packet.type === 7) {
    value = valueOf(packet.subPackets[0]) === valueOf(packet.subPackets[1]) ? BigInt(1) : BigInt(0);
  }
  return value;
}

function printPacket(packet: Packet, depth = 0) {
  console.log(depth, packet.type, packet.version, packet.value, packet.subPackets.length);
  for (let subPacket of packet.subPackets) {
    printPacket(subPacket, depth + 1);
  }
}

class Packet {
  version: number;
  type: number;
  value: bigint | null;
  subPackets: Packet[];

  constructor(version: number, type: number) {
    this.version = version;
    this.type = type;
    this.value = null;
    this.subPackets = [];
  }
}

function readPacket(start: number, str: string): [number, Packet] {
  const version = readBits(start, 3, str);
  start += 3;
  const type = readBits(start, 3, str);
  start += 3;
  const packet = new Packet(version, type);

  if (type === 4) {
    start = readLiteralPacket(start, packet, str);
  } else {
    const lengthType = readBits(start, 1, str);
    start++;
    if (lengthType === 0) {
      const length = readBits(start, 15, str);
      start += 15;
      start = readLengthPackets(start, packet, length, str);
    } else {
      const packetNum = readBits(start, 11, str);
      start += 11;
      start = readNumPackets(start, packet, packetNum, str);
    }
  }
  return [start, packet];
}

function readLengthPackets(start: number, packet: Packet, length: number, str: string): number {
  const end = start + length;
  let subPacket = packet;

  while (start < end) {
    [start, subPacket] = readPacket(start, str);
    packet.subPackets.push(subPacket);
  }

  return start;
}

function readNumPackets(start: number, packet: Packet, packetNum: number, str: string): number {
  let subPacket = packet;
  for (let i = 0; i < packetNum; i++) {
    [start, subPacket] = readPacket(start, str);
    packet.subPackets.push(subPacket);
  }

  return start;
}

function readLiteralPacket(start: number, packet: Packet, str: string): number {
  let last = readBits(start++, 1, str) === 0;
  let len = 0;
  let num = BigInt(0);
  while (true) {
    num = num * BigInt(16);
    num += BigInt(readBits(start, 4, str));
    start += 4;
    len += 4;
    if (last) break;
    last = readBits(start++, 1, str) === 0;
  }
  packet.value = num;
  return start;
}

function readBits(start: number, count: number, str: string): number {
  let startLetterIndex = Math.floor(start / 4);
  const startBit = start % 4;

  const end = start + count;
  const endLetterIndex = Math.floor(end / 4);
  const endBit = end % 4;

  let num = (Math.pow(2, 4 - startBit) - 1) & letterToNum(str.charAt(startLetterIndex));

  while (startLetterIndex < endLetterIndex) {
    startLetterIndex++;
    num = num << 4;
    num = num | letterToNum(str.charAt(startLetterIndex));
  }

  num = (num >> (4 - endBit));

  return num;
}
// 1111
// 0 = 1111
// 1 = 0111

function letterToNum(ch: string): number {
  return parseInt(ch, 16);
}

solve();
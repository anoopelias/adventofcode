import { promises as fs } from 'fs';

async function solve() {
  const input = await fs.readFile("src/2021/18/input");
  const splits = input.toString().split("\n");
  splits.pop();

  let node = stringToTree(splits[0]);
  for (let i = 1; i < splits.length; i++) {
    node = add(node, stringToTree(splits[i]));
  }

  console.log(magnitude(node));
}

function magnitude(node: Node): number {
  if (node.isLeaf()) return node.value!;
  return (3 * magnitude(node.left!)) + (2 * magnitude(node.right!));
}

function add(node1: Node, node2: Node): Node {
  const root = new Node(null, null);
  root.left = node1;
  root.right = node2;
  root.left.parent = root;
  root.right.parent = root;

  settle(root);

  return root;
}

function settle(node: Node) {
  let exploded = settleExplode(node);
  let splitted = settleSplit(node);

  let changed = exploded || splitted;
  while (changed) {
    const exploded = settleExplode(node);
    if (exploded) {
      changed = true;
      settleSplit(node);
    } else {
      changed = false;
    }
  }
}

function settleExplode(node: Node): boolean {
  let changed = false;
  let exploded = false;
  while (true) {
    exploded = explode(node);
    if (exploded) changed = true;
    if (!exploded) break;
  }
  return changed;
}

function settleSplit(node: Node): boolean {
  // ha ha :)
  let splitted = true;
  let changed = false;
  while (splitted) {
    splitted = split(node);
    settleExplode(node);
    if (splitted) changed = true;
  }

  return changed;
}

function explode(node: Node, depth = 0): boolean {
  if (node.isLeaf()) return false;
  if (depth < 4) {
    let result = explode(node.left!, depth + 1);
    if (!result) result = explode(node.right!, depth + 1);
    return result;
  }

  addLeft(node, node.left!.value!);
  addRight(node, node.right!.value!);

  node.value = 0;
  return true;
}

function split(node: Node): boolean {
  if (node.isLeaf()) {
    if (node.value! < 10) return false;
    node.left = new Node(node, Math.floor(node.value! / 2));
    node.right = new Node(node, Math.ceil(node.value! / 2));
    node.value = null;
    return true;
  }

  let res = split(node.left!);
  if (!res) res = split(node.right!);
  return res;
}

function addLeft(node: Node, value: number) {
  if (node.parent === null) return;
  if (node.parent.left === node) {
    addLeft(node.parent, value);
  } else {
    addToFirstRightLeaf(node.parent!.left!, value);
  }
}

function addToFirstRightLeaf(node: Node, value: number) {
  if (node.isLeaf()) {
    node.value = node.value! + value;
  } else {
    addToFirstRightLeaf(node.right!, value);
  }
}

function addRight(node: Node, value: number) {
  if (node.parent === null) return;
  if (node.parent.right === node) {
    addRight(node.parent, value);
  } else {
    addToFirstLeftLeaf(node.parent!.right!, value);
  }
}

function addToFirstLeftLeaf(node: Node, value: number) {
  if (node.isLeaf()) {
    node.value = node.value! + value;
  } else {
    addToFirstLeftLeaf(node.left!, value);
  }

}

function treeToString(node: Node): string {
  if (node.isLeaf()) return node.value! + "";
  return "[" + treeToString(node.left!) + "," + treeToString(node.right!) + "]";
}

function stringToTree(str: string): Node {
 
  if (str.length === 1) {
    return new Node(null, parseInt(str));
  }

  const root = new Node(null, null);
  let current: Node = root;
  for (let i = 1; i < str.length; i++) {
    if (str.charAt(i) === "[") {
      if (current.left === null) {
        current.left = new Node(current, null);
        current = current.left;
      } else {
        current.right = new Node(current, null);
        current = current.right;
      }
    } else if (str.charAt(i) === "]") {
      if (current.parent !== null) {
        current = current.parent;
      }
    } else if (str.charAt(i) === ",") {
      // Ignoring it since we already know that
      // left node is filled.
    } else {
      const value = parseInt(str.charAt(i));
      if (current.left === null) {
        current.left = new Node(current, value);
      } else {
        current.right = new Node(current, value);
      }
    }
  }
  return root;
}

class Node {
  value: number | null;
  left: Node | null;
  right: Node | null;
  parent: Node | null;

  constructor(parent: Node | null, value: number | null) {
    this.parent = parent;
    this.value = value;
    this.left = null;
    this.right = null;
  }

  isLeaf(): boolean {
    return this.value !== null;
  }
}


solve();
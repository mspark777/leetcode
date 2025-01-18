import "@total-typescript/ts-reset";

class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}

function solve(root: TreeNode | null, tail: TreeNode | null): TreeNode | null {
  if (root == null) {
    return tail;
  }

  const res = solve(root.left, root);
  root.left = null;
  root.right = solve(root.right, tail);
  return res;
}

function increasingBST(root: TreeNode | null): TreeNode | null {
  return solve(root, null);
}

function main(): void {
  const inputs: Array<string[]> = [
    ["a", "aba", "ababa", "aa"],
    ["pa", "papa", "ma", "mama"],
    ["abab", "ab"],
  ];

  for (const words of inputs) {
    const result = countPrefixSuffixPairs(words);
    console.log(result);
  }
}
main();

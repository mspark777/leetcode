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

function dfs(
  left: TreeNode | null,
  right: TreeNode | null,
  level: number,
): void {
  if (left == null || right == null) {
    return;
  }

  if (level % 2 == 0) {
    const temp = left.val;
    left.val = right.val;
    right.val = temp;
  }

  dfs(left.left, right.right, level + 1);
  dfs(left.right, right.left, level + 1);
}

function reverseOddLevels(root: TreeNode | null): TreeNode | null {
  if (root == null) {
    return null;
  }

  dfs(root.left, root.right, 0);
  return root;
}

function main(): void {
  const inputs: Array<number[]> = [
    [4, 3, 6, 16, 8, 2],
    [2, 3, 5, 6, 7],
  ];

  for (const nums of inputs) {
    const result = longestSquareStreak(nums);
    console.log(result);
  }
}
main();

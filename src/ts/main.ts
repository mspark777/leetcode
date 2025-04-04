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

function dfs(root: TreeNode | null): [TreeNode | null, number] {
  if (root == null) {
    return [null, 0];
  }

  const [left, leftDepth] = dfs(root.left);
  const [right, rightDepth] = dfs(root.right);

  if (leftDepth > rightDepth) {
    return [left, leftDepth + 1];
  } else if (leftDepth < rightDepth) {
    return [right, rightDepth + 1];
  }

  return [root, leftDepth + 1];
}

function lcaDeepestLeaves(root: TreeNode | null): TreeNode | null {
  const [node] = dfs(root);
  return node;
}

interface Input {
  points: number[][];
}

function main(): void {
  const inputs: Input[] = [
    {
      points: [
        [1, 1],
        [2, 3],
        [3, 2],
      ],
    },
    {
      points: [
        [1, 1],
        [2, 2],
        [3, 3],
      ],
    },
  ];

  for (const input of inputs) {
    const result = isBoomerang(input.points);
    console.log(result);
  }
}
main();

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

interface Result {
  n: number;
}

function solve(node: TreeNode, sum: number, result: Result): void {
  const n = (sum << 1) | node.val;
  if (node.left == null && node.right == null) {
    result.n += n;
  } else {
    if (node.left != null) {
      solve(node.left, n, result);
    }

    if (node.right != null) {
      solve(node.right, n, result);
    }
  }
}

function sumRootToLeaf(root: TreeNode | null): number {
  if (root == null) {
    return 0;
  }

  const result: Result = { n: 0 };
  solve(root, 0, result);
  return result.n;
}

interface Input {
  edges: number[][];
  bob: number;
  amount: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      edges: [
        [0, 1],
        [1, 2],
        [1, 3],
        [3, 4],
      ],
      bob: 3,
      amount: [-2, 4, 2, -4, 6],
    },
    {
      edges: [[0, 1]],
      bob: 1,
      amount: [-7280, 2350],
    },
  ];

  for (const input of inputs) {
    const result = mostProfitablePath(input.edges, input.bob, input.amount);
    console.log(result);
  }
}
main();

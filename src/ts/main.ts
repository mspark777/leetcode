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

interface Index {
  pre: number;
  post: number;
}

function constructTree(
  idx: Index,
  preorder: number[],
  postorder: number[],
): TreeNode | null {
  if (idx.pre >= preorder.length) {
    return null;
  }

  const root = new TreeNode(preorder[idx.pre]);
  idx.pre += 1;

  if (idx.post >= postorder.length) {
    return null;
  }

  if (root.val !== postorder[idx.post]) {
    root.left = constructTree(idx, preorder, postorder);
  }

  if (root.val !== postorder[idx.post]) {
    root.right = constructTree(idx, preorder, postorder);
  }

  idx.post += 1;
  return root;
}

function constructFromPrePost(
  preorder: number[],
  postorder: number[],
): TreeNode | null {
  const idx: Index = { pre: 0, post: 0 };
  return constructTree(idx, preorder, postorder);
}

interface Input {
  preorder: number[];
  postorder: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      preorder: [1, 2, 4, 5, 3, 6, 7],
      postorder: [4, 5, 2, 6, 7, 3, 1],
    },
    {
      preorder: [1],
      postorder: [4],
    },
  ];

  for (const input of inputs) {
    const result = constructFromPrePost(input.preorder, input.postorder);
    console.log(result);
  }
}
main();

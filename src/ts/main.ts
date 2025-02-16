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

function solve(node: TreeNode, val: number): boolean {
  if (node.val !== val) {
    return false;
  }

  if (node.left != null) {
    if (!solve(node.left, val)) {
      return false;
    }
  }

  if (node.right != null) {
    if (!solve(node.right, val)) {
      return false;
    }
  }

  return true;
}

function isUnivalTree(root: TreeNode | null): boolean {
  if (root == null) {
    return false;
  }

  const val = root.val;
  return solve(root, val);
}

function main(): void {
  const inputs: Array<string[]> = [
    [
      "test.email+alex@leetcode.com",
      "test.e.mail+bob.cathy@leetcode.com",
      "testemail+david@lee.tcode.com",
    ],
    ["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"],
  ];

  for (const email of inputs) {
    const result = numUniqueEmails(email);
    console.log(result);
  }
}
main();

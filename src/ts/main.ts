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

class FindElements {
  seen: Set<number>;
  constructor(root: TreeNode | null) {
    this.seen = new Set();
    if (root == null) {
      return;
    }

    const stack: Array<[TreeNode, number]> = [[root, 0]];
    while (stack.length > 0) {
      const [node, val] = stack.pop() as [TreeNode, number];
      this.seen.add(val);
      if (node.left != null) {
        stack.push([node.left, val * 2 + 1]);
      }

      if (node.right != null) {
        stack.push([node.right, val * 2 + 2]);
      }
    }
  }

  find(target: number): boolean {
    return this.seen.has(target);
  }
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

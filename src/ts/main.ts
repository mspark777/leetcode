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

interface Node {
  node: TreeNode;
  parent: TreeNode;
  level: number;
}

function findNode(root: TreeNode, val: number): Node {
  const stack: Node[] = [
    {
      node: root,
      parent: root,
      level: 0,
    },
  ];

  while (stack.length > 0) {
    const { node, parent, level } = stack.pop() as Node;
    if (node.val == val) {
      return { node, parent, level };
    }

    if (node.left != null) {
      stack.push({ node: node.left, parent: node, level: level + 1 });
    }

    if (node.right != null) {
      stack.push({ node: node.right, parent: node, level: level + 1 });
    }
  }

  return null as unknown as Node;
}

function isCousins(root: TreeNode | null, x: number, y: number): boolean {
  if (root == null) {
    return false;
  }

  const xNode = findNode(root, x);
  if (xNode.level === 0) {
    return false;
  }

  const yNode = findNode(root, y);
  if (yNode.level === 0) {
    return false;
  } else if (xNode.level !== yNode.level) {
    return false;
  }

  return xNode.parent.val !== yNode.parent.val;
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

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

function recoverFromPreorder(traversal: string): TreeNode | null {
  const levels: TreeNode[] = [];
  const n = traversal.length;
  let i = 0;
  while (i < n) {
    let depth = 0;
    while (i < n) {
      if (traversal.at(i) === "-") {
        depth += 1;
        i += 1;
      } else {
        break;
      }
    }

    let value = 0;
    while (i < n) {
      const code = traversal.charCodeAt(i) - "0".charCodeAt(0);
      if (0 <= code && code <= 9) {
        value = value * 10 + code;
        i += 1;
      } else {
        break;
      }
    }

    const node = new TreeNode(value);
    if (depth < levels.length) {
      levels[depth] = node;
    } else {
      levels.push(node);
    }

    if (depth > 0) {
      const parent = levels[depth - 1] as TreeNode;
      if (parent.left == null) {
        parent.left = node;
      } else {
        parent.right = node;
      }
    }
  }

  return levels.at(0) as TreeNode;
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

/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

import { toTree } from "./tree_node.mjs";

/**
 * @param {TreeNode|null} root
 * @return {number}
 */
function sumOfLeftLeaves(root) {
  if (root == null) {
    return 0;
  }

  let result = 0;

  const stack = [{ node: root, left: false }];
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    let leaf = true;
    const { node } = top;
    if (node.left != null) {
      leaf = false;
      stack.push({ node: node.left, left: true });
    }

    if (node.right != null) {
      leaf = false;
      stack.push({ node: node.right, left: false });
    }

    if (leaf && top.left) {
      result += node.val;
    }
  }

  return result;
}

function main() {
  const inputs = [[3, 9, 20, null, null, 15, 7], [1]];

  for (const nums of inputs) {
    const result = sumOfLeftLeaves(toTree(nums));
    console.log(result);
  }
}
main();

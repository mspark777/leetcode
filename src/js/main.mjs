/* eslint-disable @typescript-eslint/explicit-function-return-type */

import { TreeNode } from "./tree_node.mjs";

class Solution {
  /** @type {number} */
  result;
  constructor() {
    this.result = 0;
  }

  /**
   * @param {TreeNode|null} node
   * @return {number}
   */
  resolve(node) {
    if (node == null) {
      return 0;
    }

    const ldepth = this.resolve(node.left);
    const rdepth = this.resolve(node.right);
    this.result = Math.max(this.result, ldepth + rdepth);
    return 1 + Math.max(ldepth, rdepth);
  }

  /**
   * @param {TreeNode|null} root
   * @return {number}
   */
  diameterOfBinaryTree(root) {
    this.resolve(root);
    return this.result;
  }
}

/**
 * @param {TreeNode|null} [root]
 * @return {number}
 */
function diameterOfBinaryTree(root) {
  const solution = new Solution();
  return solution.diameterOfBinaryTree(root);
}

function main() {
  const inputs = [
    new TreeNode(
      1,
      new TreeNode(2, new TreeNode(4), new TreeNode(5)),
      new TreeNode(3),
    ),
    new TreeNode(1, new TreeNode(2)),
  ];

  for (const root of inputs) {
    const result = diameterOfBinaryTree(root);
    console.log(result);
  }
}
main();

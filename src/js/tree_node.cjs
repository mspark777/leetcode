class TreeNode {
  /** @type {number} */
  val;
  /** @type {TreeNode|null} */
  left;
  /** @type {TreeNode|null} */
  right;

  /**
   * @param {number} val
   * @param {TreeNode|null} [left]
   * @param {TreeNode|null} [right]
   */
  constructor(val, left, right) {
    this.val = val ?? 0;
    this.left = left ?? null;
    this.right = right ?? null;
  }
}
module.exports = TreeNode;

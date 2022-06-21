/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 */
export const BSTIterator = function (root) {
  this.cur = root
  this.stack = []

  this.emptyNumber = -1
  while (root) {
    if (!root.left) {
      this.emptyNumber = root.val - 1
    }
    root = root.left
  }
}

/**
 * @return {number}
 */
BSTIterator.prototype.next = function () {
  while (this.cur) {
    this.stack.push(this.cur)
    this.cur = this.cur.left
  }

  if (this.stack.length < 1) {
    return this.emptyNumber
  }

  const node = this.stack.pop()
  this.cur = node.right
  return node.val
}

/**
 * @return {boolean}
 */
BSTIterator.prototype.hasNext = function () {
  return (this.stack.length > 0) || (!!this.cur)
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * var obj = new BSTIterator(root)
 * var param_1 = obj.next()
 * var param_2 = obj.hasNext()
 */

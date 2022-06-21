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
export var BSTIterator = function (root) {
  this.cur = root
  this.stack = []
}

/**
 * @return {number}
 */
BSTIterator.prototype.next = function () {
  while (this.cur) {
    this.stack.push(this.cur)
    this.cur = this.cur.left
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

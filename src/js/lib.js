export class TreeNode {
  /** @type {number} */
  val
  /** @type {TreeNode | null} */
  left
  /** @type {TreeNode | null} */
  right

  /**
    * @param {number | undefined} val
    * @param {TreeNode | null | undefined} left
    * @param {TreeNode | null | undefined} right
    */
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

/**
  * @param {number} val
  * @param {TreeNode} left
  * @param {TreeNode} right
  * @returns {TreeNode}
  */
export function newTreeNode (val, left, right) {
  return new TreeNode(val, left, right)
}

/**
  * @param {number} val
  * @param {TreeNode} left
  * @returns {TreeNode}
  */
export function newTreeLeft (val, left) {
  return new TreeNode(val, left)
}

/**
  * @param {number} val
  * @param {TreeNode} right
  * @returns {TreeNode}
  */
export function newTreeRight (val, right) {
  return new TreeNode(val, undefined, right)
}

/**
  * @param {number} val
  * @returns {TreeNode}
  */
export function newTreeVal (val) {
  return new TreeNode(val)
}

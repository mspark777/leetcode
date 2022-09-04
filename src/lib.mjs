
export class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function newTreeNode (val, left, right) {
  return new TreeNode(val, left, right)
}

export function newTreeVal (val) {
  return newTreeNode(val, null, null)
}

export function newTreeLeft (val, left) {
  return newTreeNode(val, left, null)
}

export function newTreeRight (val, right) {
  return newTreeNode(val, null, right)
}

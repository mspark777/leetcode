export class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function newTreeNode (val: number, left: TreeNode | null, right: TreeNode | null): TreeNode {
  return new TreeNode(val, left, right)
}

export function newTreeVal (val: number): TreeNode {
  return newTreeNode(val, null, null)
}

export function newTreeLeft (val: number, left: TreeNode): TreeNode {
  return newTreeNode(val, left, null)
}

export function newTreeRight (val: number, right: TreeNode): TreeNode {
  return newTreeNode(val, null, right)
}

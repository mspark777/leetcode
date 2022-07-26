export class TreeNode {
  val: number
  // eslint-disable-next-line no-use-before-define
  left: TreeNode | null
  // eslint-disable-next-line no-use-before-define
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function preorder (node: TreeNode | null, result: number[]): void {
  if (node) {
    result.push(node.val)
    preorder(node.left, result)
    preorder(node.right, result)
  }
}

export function preorderTraversal (root: TreeNode | null): number[] {
  const result: number[] = []
  preorder(root, result)
  return result
}

export class TreeNode {
  val: number
  // eslint-disable-next-line no-use-before-define
  left: TreeNode | null
  // eslint-disable-next-line no-use-before-define
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val ?? 0
    this.left = left ?? null
    this.right = right ?? null
  }
}

export function hasPathSum (root: TreeNode | null, targetSum: number): boolean {
  if (!root) {
    return false
  } else if (root.left === null && root.right === null) {
    return root.val === targetSum
  }

  targetSum -= root.val
  return hasPathSum(root.left, targetSum) || hasPathSum(root.right, targetSum)
}

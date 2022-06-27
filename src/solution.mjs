export class TreeNode {
  constructor (val, left, right) {
    this.val = val ?? 0
    this.left = left ?? null
    this.right = right ?? null
  }
}

export function hasPathSum (root, targetSum) {
  if (!root) {
    return false
  } else if (root.left === null && root.right === null) {
    return root.val === targetSum
  }

  targetSum -= root.val
  return hasPathSum(root.left, targetSum) || hasPathSum(root.right, targetSum)
}

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

export function lowestCommonAncestor (root: TreeNode | null, p: TreeNode | null, q: TreeNode | null): TreeNode | null {
  if (!root || (root === p) || (root === q)) {
    return root
  }

  const left = lowestCommonAncestor(root.left, p, q)
  const right = lowestCommonAncestor(root.right, p, q)

  if (!left) {
    return right
  } else if (!right) {
    return left
  } else {
    return root
  }
}

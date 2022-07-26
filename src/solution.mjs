export class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function lowestCommonAncestor (root, p, q) {
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

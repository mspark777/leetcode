export class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function flatten (root) {
  while (root) {
    if (root.left) {
      const right = root.right
      let predecessor = root.left
      while (predecessor.right) {
        predecessor = predecessor.right
      }

      predecessor.right = right
      root.right = root.left
      root.left = null
    }
    root = root.right
  }
}

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

export function flatten (root: TreeNode | null): void {
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

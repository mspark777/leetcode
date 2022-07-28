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

export function postorderTraversal (root: TreeNode | null): number[] {
  if (!root) {
    return []
  }

  const result: number[] = []
  const stack: TreeNode[] = [root]
  for (let node = stack.pop(); node; node = stack.pop()) {
    result.push(node.val)

    const left = node.left
    if (left) {
      stack.push(left)
    }

    const right = node.right
    if (right) {
      stack.push(right)
    }
  }

  return result.reverse()
}

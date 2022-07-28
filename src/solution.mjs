export class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function postorderTraversal (root) {
  if (!root) {
    return []
  }

  const result = []
  const stack = [root]
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

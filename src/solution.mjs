export class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function levelOrder (root) {
  if (!root) {
    return []
  }

  const result = []
  const queue = [root]

  while (queue.length > 0) {
    const count = queue.length
    const level = []
    for (let i = 0; i < count; i += 1) {
      const node = queue.shift()
      level.push(node.val)

      if (node.left) {
        queue.push(node.left)
      }

      if (node.right) {
        queue.push(node.right)
      }
    }

    result.push(level)
  }

  return result
}

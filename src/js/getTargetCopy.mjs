
// eslint-disable-next-line @typescript-eslint/no-unused-vars
class TreeNode {
  constructor (val) {
    this.val = val
    this.left = this.right = null
  }
}

/**
 * @param {TreeNode} original
 * @param {TreeNode} cloned
 * @param {TreeNode} target
 * @return {TreeNode}
 */
export const getTargetCopy = function (_original, cloned, target) {
  const stack = [cloned]
  while (stack.length > 0) {
    const top = stack.pop()
    if (!top) {
      continue
    } if (top.val === target.val) {
      return top
    }

    stack.push(top.left)
    stack.push(top.right)
  }

  return null
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

/**
 * @param {TreeNode} root
 * @param {number} low
 * @param {number} high
 * @return {TreeNode}
 */
export const trimBST1 = function (root, low, high) {
  if (!root) {
    return root
  }

  if (root.val < low) {
    return trimBST1(root.right, low, high)
  } else if (root.val > high) {
    return trimBST1(root.left, low, high)
  }

  root.left = trimBST1(root.left, low, high)
  root.right = trimBST1(root.right, low, high)
  return root
}

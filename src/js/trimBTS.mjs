
class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

/**
 * @param {TreeNode} root
 * @param {TreeNode} current
 * @param {string} direction
 * @param {number} low
 * @param {number} high
 * @return {undefined}
 */
function travel (root, current, direction, low, high) {
  if (!root) {
    return
  }

  if (root.val < low) {
    travel(root.right, current, direction, low, high)
  } else if (root.val > high) {
    travel(root.left, current, direction, low, high)
  } else {
    current[direction] = new TreeNode(root.val)
    travel(root.left, current[direction], 'left', low, high)
    travel(root.right, current[direction], 'right', low, high)
  }
}

/**
 * @param {TreeNode} root
 * @param {number} low
 * @param {number} high
 * @return {TreeNode | undefined}
 */
function findRootInRange (root, low, high) {
  if (!root) {
    return
  }

  if (root.val < low) {
    return findRootInRange(root.right, low, high)
  } else if (root.val > high) {
    return findRootInRange(root.left, low, high)
  }

  return root
}

/**
 * @param {TreeNode} root
 * @param {number} low
 * @param {number} high
 * @return {TreeNode}
 */
export const trimBST = function (root, low, high) {
  const inRangeRoot = findRootInRange(root, low, high)
  if (!inRangeRoot) {
    return null
  }

  const current = new TreeNode(inRangeRoot.val)
  travel(inRangeRoot.left, current, 'left', low, high)
  travel(inRangeRoot.right, current, 'right', low, high)

  return current
}

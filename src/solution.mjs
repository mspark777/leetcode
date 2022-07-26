export class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function preorder (node, result) {
  if (node) {
    result.push(node.val)
    preorder(node.left, result)
    preorder(node.right, result)
  }
}

export function preorderTraversal (root) {
  const result = []
  preorder(root, result)
  return result
}

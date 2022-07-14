class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function buildTree (preorder, inorder) {
  preorder.reverse()
  inorder.reverse()

  const build = bound => {
    if (
      (inorder.length < 1) ||
      ((bound !== null) && (inorder.at(-1) === bound))
    ) {
      return null
    }

    const node = new TreeNode(preorder.pop())
    node.left = build(node.val)

    inorder.pop()
    node.right = build(bound)

    return node
  }
  return build(null)
}

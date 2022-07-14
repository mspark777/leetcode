class TreeNode {
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

export function buildTree (preorder: number[], inorder: number[]): TreeNode | null {
  preorder.reverse()
  inorder.reverse()

  const build = (bound: number | null): TreeNode | null => {
    if (
      (inorder.length < 1) ||
      ((bound !== null) && (inorder.at(-1) === bound))
    ) {
      return null
    }

    const node = new TreeNode(preorder.pop()!)
    node.left = build(node.val)

    inorder.pop()
    node.right = build(bound)

    return node
  }
  return build(null)
}

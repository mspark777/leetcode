import { newTreeNode, newTreeRight, newTreeVal, type TreeNode } from './lib'

function isMirror (left?: TreeNode | null, right?: TreeNode | null): boolean {
  if ((left == null) && (right == null)) {
    return true
  } else if ((left == null) || (right == null)) {
    return false
  }

  return (left.val === right.val) &&
    isMirror(left.left, right.right) &&
    isMirror(left.right, right.left)
}

function isSymmetric (root: TreeNode | null): boolean {
  return isMirror(root?.left, root?.right)
}

async function main (): Promise<void> {
  const inputs: Array<TreeNode | null> = [
    newTreeNode(1, newTreeNode(2, newTreeVal(3), newTreeVal(4)), newTreeNode(2, newTreeVal(4), newTreeVal(3))),
    newTreeNode(1, newTreeRight(2, newTreeVal(3)), newTreeRight(2, newTreeVal(3)))
  ]

  for (const root of inputs) {
    const result = isSymmetric(root)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })

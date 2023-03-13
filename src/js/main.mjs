import { newTreeNode, newTreeRight, newTreeVal, TreeNode, unused } from './lib.mjs'
unused(TreeNode)

/**
  * @param {TreeNode|undefined|null} left
  * @param {TreeNode|undefined|null} right
  * @returns {boolean}
  */
function isMirror (left, right) {
  if ((left == null) && (right == null)) {
    return true
  } else if ((left == null) || (right == null)) {
    return false
  }

  return (left.val === right.val) &&
    isMirror(left.left, right.right) &&
    isMirror(left.right, right.left)
}

/**
  * @param {TreeNode|null} root
  * @returns {boolean}
  */
function isSymmetric (root) {
  return isMirror(root?.left, root?.right)
}

async function main () {
  const inputs = [
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

import { newTreeNode, newTreeLeft, newTreeRight, newTreeVal, TreeNode, unused } from './lib.mjs'
unused(TreeNode)

/**
  * @param {TreeNode|null} root
  * @retrns {boolean]
  */
function isCompleteTree (root) {
  if (root == null) {
    return true
  }

  let nullFound = false
  /** @type {Array<TreeNode|null>} */
  const queue = [root]

  while (queue.length > 0) {
    const node = queue.shift()

    if (node == null) {
      nullFound = true
      continue
    }

    if (nullFound) {
      return false
    }

    queue.push(node.left, node.right)
  }

  return true
}

async function main () {
  const inputs = [
    newTreeNode(1, newTreeNode(2, newTreeVal(4), newTreeVal(5)), newTreeLeft(3, newTreeVal(6))),
    newTreeNode(1, newTreeNode(2, newTreeVal(4), newTreeVal(5)), newTreeRight(3, newTreeVal(7)))
  ]

  for (const root of inputs) {
    const result = isCompleteTree(root)
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

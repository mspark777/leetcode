import { newTreeLeft, newTreeNode, newTreeRight, newTreeVal, TreeNode, unused } from './utils.mjs'
unused(TreeNode)

/**
  * @param {TreeNode | null} root
  * @returns {number}
  */
function widthOfBinaryTree (root) {
  if (root == null) {
    return 0
  }

  let result = 1
  /** @type {Array<[TreeNode, bigint]>} */
  let queue = [[root, 0n]]
  while (queue.length > 0) {
    const count = queue.length
    const start = queue[0][1]
    const end = queue[count - 1][1]
    result = Math.max(result, Number(end - start + 1n))
    for (let i = 0; i < count; i += 1) {
      const [node, nodeIdx] = queue[i]
      const idx = nodeIdx - start
      const left = node.left
      const right = node.right

      if (left != null) {
        queue.push([left, 2n * idx + 1n])
      }

      if (right != null) {
        queue.push([right, 2n * (idx + 1n)])
      }
    }

    queue = queue.slice(count)
  }

  return result
}

async function main () {
  const inputs = [
    newTreeNode(1, newTreeNode(3, newTreeVal(5), newTreeVal(3)), newTreeRight(2, newTreeVal(9))),
    newTreeNode(1, newTreeLeft(3, newTreeLeft(5, newTreeVal(6))), newTreeRight(2, newTreeLeft(9, newTreeVal(7)))),
    newTreeNode(1, newTreeLeft(3, newTreeVal(5)), newTreeVal(2))
  ]

  for (const root of inputs) {
    const result = widthOfBinaryTree(root)
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

import { newTreeNode, newTreeVal, TreeNode, unused } from './lib.mjs'
unused(TreeNode)

/**
  * @param {TreeNode|null} node
  * @param {bigint} sum
  * @returns {bigint}
  */
function travel (node, sum) {
  if (node == null) {
    return 0n
  }

  const newsum = (sum * 10n) + BigInt(node.val)
  const { left, right } = node
  if ((left == null) && (right == null)) {
    return newsum
  }

  return travel(left, newsum) + travel(right, newsum)
}

/**
  * @param {TreeNode|null} root
  * @returns {number}
  */
function sumNumbers (root) {
  const result = travel(root, 0n)
  return Number(result)
}

async function main () {
  const inputs = [
    newTreeNode(1, newTreeVal(2), newTreeVal(3)),
    newTreeNode(4, newTreeNode(9, newTreeVal(5), newTreeVal(1)), newTreeVal(0))
  ]

  for (const root of inputs) {
    const result = sumNumbers(root)
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

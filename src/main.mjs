import { newTreeLeft, newTreeNode, newTreeVal } from './lib.mjs'

/**
  * @param {TreeNode|null} node
  * @param {Map<string, number>} tripletToIDs
  * @param {Map<number, number>} counts
  * @param {TreeNode[]} result
  * @returns {number}
  */
function traverse (node, tripletToIDs, counts, result) {
  if (node == null) {
    return 0
  }

  const triplet = [
    traverse(node.left, tripletToIDs, counts, result),
    node.val,
    traverse(node.right, tripletToIDs, counts, result)
  ].join(',')

  if (!tripletToIDs.has(triplet)) {
    tripletToIDs.set(triplet, tripletToIDs.size + 1)
  }
  const id = tripletToIDs.get(triplet)
  counts.set(id, (counts.get(id) ?? 0) + 1)
  if (counts.get(id) === 2) {
    result.push(node)
  }

  return id
}

/**
  * @param {TreeNode|null} root
  * @returns {Array<TreeNode | null>}
  */
function findDuplicateSubtrees (root) {
  /** @type {TreeNode[]} */
  const result = []
  traverse(root, new Map(), new Map(), result)

  return result
}

async function main () {
  const inputs = [
    newTreeNode(1,
      newTreeLeft(2, newTreeVal(4)),
      newTreeNode(3, newTreeLeft(2, newTreeVal(4)), newTreeVal(4))
    ),
    newTreeNode(2, newTreeVal(1), newTreeVal(1)),
    newTreeNode(2, newTreeLeft(2, newTreeVal(3)), newTreeLeft(2, newTreeVal(3)))
  ]

  for (const root of inputs) {
    const result = findDuplicateSubtrees(root)
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

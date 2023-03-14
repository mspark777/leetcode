import { newTreeNode, newTreeVal, type TreeNode } from './lib'

function travel (node: TreeNode | null, sum: bigint): bigint {
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

function sumNumbers (root: TreeNode | null): number {
  const result = travel(root, 0n)
  return Number(result)
}

async function main (): Promise<void> {
  const inputs: Array<TreeNode | null> = [
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

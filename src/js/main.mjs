import { newTreeNode, newTreeRight, newTreeVal, TreeNode, unused } from './utils.mjs'
unused(TreeNode)

class DFS {
  /** @type {number} */
  maxSteps
  constructor () {
    this.maxSteps = 0
  }

  /**
    * @param {TreeNode | null} node
    * @param {boolean} left
    * @param {number} steps
    * @returns {undefined}
    */
  dfs (node, left, steps) {
    if (node == null) {
      return
    }

    this.maxSteps = Math.max(this.maxSteps, steps)
    if (left) {
      this.dfs(node.left, false, steps + 1)
      this.dfs(node.right, true, 1)
    } else {
      this.dfs(node.left, false, 1)
      this.dfs(node.right, true, steps + 1)
    }
  }
}

/**
  * @param {TreeNode | null} root
  * @returns {number}
  */
function longestZigZag (root) {
  const dfs = new DFS()
  dfs.dfs(root, true, 0)
  dfs.dfs(root, false, 0)

  return dfs.maxSteps
}

async function main () {
  const inputs = [
    newTreeRight(1, newTreeNode(1, newTreeVal(1), newTreeNode(1, newTreeRight(1, newTreeRight(1, newTreeVal(1))), newTreeVal(1)))),
    newTreeNode(1, newTreeRight(1, newTreeNode(1, newTreeRight(1, newTreeVal(1)), newTreeVal(1))), newTreeVal(1)),
    newTreeVal(1)
  ]

  for (const root of inputs) {
    const result = longestZigZag(root)
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

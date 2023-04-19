import { newTreeNode, newTreeRight, newTreeVal, type TreeNode } from './utils'

class DFS {
  public maxSteps: number
  public constructor () {
    this.maxSteps = 0
  }

  public dfs (node: TreeNode | null, left: boolean, steps: number): void {
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

function longestZigZag (root: TreeNode | null): number {
  const dfs = new DFS()
  dfs.dfs(root, true, 0)
  dfs.dfs(root, false, 0)

  return dfs.maxSteps
}

async function main (): Promise<void> {
  const inputs: Array<TreeNode | null> = [
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

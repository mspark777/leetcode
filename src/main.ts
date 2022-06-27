import { hasPathSum, TreeNode } from './solution'

interface Input {
  readonly root: TreeNode | null
  readonly targetSum: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      targetSum: 22,
      root: new TreeNode(5,
        new TreeNode(4,
          new TreeNode(11,
            new TreeNode(7),
            new TreeNode(2)
          )
        ),
        new TreeNode(8,
          new TreeNode(13),
          new TreeNode(4,
            null,
            new TreeNode(1)
          )
        )
      )
    },
    {
      targetSum: 5,
      root: new TreeNode(1,
        new TreeNode(2),
        new TreeNode(3)
      )
    },
    {
      targetSum: 0,
      root: null
    }
  ]

  for (const input of inputs) {
    const result = hasPathSum(input.root, input.targetSum)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

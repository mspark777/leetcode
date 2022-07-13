import { levelOrder, TreeNode } from './solution'

interface Input {
  readonly root: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: new TreeNode(3,
        new TreeNode(9),
        new TreeNode(20,
          new TreeNode(15),
          new TreeNode(7)
        )
      )
    },
    {
      root: new TreeNode(1)
    },
    {
      root: null
    }
  ]

  for (const input of inputs) {
    const result = levelOrder(input.root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

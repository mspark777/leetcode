import { preorderTraversal, TreeNode } from './solution'

interface Input {
  readonly root: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: new TreeNode(1,
        null,
        new TreeNode(2,
          new TreeNode(3)
        )
      )
    },
    {
      root: null
    },
    {
      root: new TreeNode(1)
    }
  ]

  for (const input of inputs) {
    const result = preorderTraversal(input.root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

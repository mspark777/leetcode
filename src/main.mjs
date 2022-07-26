// import { createRequire } from 'module'
import { preorderTraversal, TreeNode } from './solution.mjs'

async function main () {
  const inputs = [
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

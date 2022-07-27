// import { createRequire } from 'module'
import { flatten, TreeNode } from './solution.mjs'

function treetoarr (node) {
  const nums = []
  while (node) {
    nums.push(node.val)
    node = node.right
  }

  return nums
}

async function main () {
  const inputs = [
    {
      root: new TreeNode(1,
        new TreeNode(2,
          new TreeNode(3),
          new TreeNode(4)
        ),
        new TreeNode(5,
          null,
          new TreeNode(8)
        )
      )
    },
    {
      root: null
    },
    {
      root: new TreeNode(0)
    }
  ]

  for (const input of inputs) {
    flatten(input.root)
    console.log(treetoarr(input.root))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

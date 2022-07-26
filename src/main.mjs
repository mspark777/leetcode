// import { createRequire } from 'module'
import { lowestCommonAncestor, TreeNode } from './solution.mjs'

function findNode (node, val) {
  if (!node) {
    return node
  } else if (node.val === val) {
    return node
  }

  return findNode(node.left, val) ?? findNode(node.right, val)
}

async function main () {
  const inputs = [
    {
      root: new TreeNode(3,
        new TreeNode(5,
          new TreeNode(6),
          new TreeNode(2,
            new TreeNode(7),
            new TreeNode(4)
          )
        ),
        new TreeNode(1,
          new TreeNode(0),
          new TreeNode(8)
        )
      ),
      p: null,
      q: null,
      pv: 5,
      qv: 1
    },
    {
      root: new TreeNode(3,
        new TreeNode(5,
          new TreeNode(6),
          new TreeNode(2,
            new TreeNode(7),
            new TreeNode(4)
          )
        ),
        new TreeNode(1,
          new TreeNode(0),
          new TreeNode(8)
        )
      ),
      p: null,
      q: null,
      pv: 5,
      qv: 4
    },
    {
      root: new TreeNode(1, new TreeNode(2)),
      p: null,
      q: null,
      pv: 1,
      qv: 2
    }
  ]

  for (const input of inputs) {
    input.q = findNode(input.root, input.qv)
    input.p = findNode(input.root, input.pv)
    const result = lowestCommonAncestor(input.root, input.p, input.q)
    console.log(result?.val)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

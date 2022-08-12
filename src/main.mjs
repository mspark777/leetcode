class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
function lowestCommonAncestor (root, p, q) {
  if (!p || !q) {
    return null
  }

  const pval = p.val
  const qval = q.val

  let cur = root
  while (cur) {
    const val = cur.val
    if ((pval < val) && (qval < val)) {
      cur = cur.left
    } else if ((pval > val) && (qval > val)) {
      cur = cur.right
    } else {
      break
    }
  }

  return cur
}

function getNode (root, val) {
  if (!root) {
    return null
  }

  if (root.val === val) {
    return root
  }

  return getNode(root.left, val) ?? getNode(root.right, val)
}

async function main () {
  const inputs = [
    {
      root: new TreeNode(6,
        new TreeNode(2,
          new TreeNode(0),
          new TreeNode(4,
            new TreeNode(3),
            new TreeNode(5)
          )
        ),
        new TreeNode(8,
          new TreeNode(7),
          new TreeNode(9)
        )
      ),
      pval: 2,
      qval: 8
    },
    {
      root: new TreeNode(6,
        new TreeNode(2,
          new TreeNode(0),
          new TreeNode(4,
            new TreeNode(3),
            new TreeNode(5)
          )
        ),
        new TreeNode(8,
          new TreeNode(7),
          new TreeNode(9)
        )
      ),
      pval: 2,
      qval: 4
    },
    {
      root: new TreeNode(2,
        new TreeNode(1)
      ),
      pval: 2,
      qval: 1
    }
  ]

  for (const { root, pval, qval } of inputs) {
    const p = getNode(root, pval)
    const q = getNode(root, qval)
    const result = lowestCommonAncestor(root, p, q)
    console.log(result?.val)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

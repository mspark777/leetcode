class TreeNode {
  val: number
  // eslint-disable-next-line no-use-before-define
  left: TreeNode | null
  // eslint-disable-next-line no-use-before-define
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function lowestCommonAncestor (root: TreeNode | null, p: TreeNode | null, q: TreeNode | null): TreeNode | null {
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

interface Input {
  root: TreeNode | null
  qval: number
  pval: number
}

function getNode (root: TreeNode | null, val: number): TreeNode | null {
  if (!root) {
    return null
  }

  if (root.val === val) {
    return root
  }

  return getNode(root.left, val) ?? getNode(root.right, val)
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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

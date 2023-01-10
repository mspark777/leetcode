class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function newnode (val: number, left: TreeNode | null, right: TreeNode | null): TreeNode {
  return new TreeNode(val, left, right)
}

function newleft (val: number, left: TreeNode): TreeNode {
  return newnode(val, left, null)
}

function newright (val: number, right: TreeNode): TreeNode {
  return newnode(val, null, right)
}

function newval (val: number): TreeNode {
  return newnode(val, null, null)
}

function isSameTree (p: TreeNode | null, q: TreeNode | null): boolean {
  if ((p == null) && (q == null)) {
    return true
  }

  if ((p == null) || (q == null)) {
    return false
  }

  return p.val === q.val
    ? isSameTree(p.left, q.left) && isSameTree(p.right, q.right)
    : false
}

async function main (): Promise<void> {
  const inputs: TreeNode[][] = [
    [
      newnode(1, newval(2), newval(3)),
      newnode(1, newval(2), newval(3))
    ],
    [
      newleft(1, newval(2)),
      newright(1, newval(2))
    ],
    [
      newnode(1, newval(2), newval(1)),
      newnode(1, newval(1), newval(2))
    ]
  ]

  for (const [p, q] of inputs) {
    const result = isSameTree(p, q)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

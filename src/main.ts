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

function newnode (val: number, left: TreeNode, right: TreeNode): TreeNode {
  return new TreeNode(val, left, right)
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function newleft (val: number, left: TreeNode): TreeNode {
  return new TreeNode(val, left)
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function newright (val: number, right: TreeNode): TreeNode {
  return new TreeNode(val, undefined, right)
}

function newval (val: number): TreeNode {
  return new TreeNode(val)
}

function travel (node: TreeNode | null, curmax: number, curmin: number): number {
  if (node == null) {
    return curmax - curmin
  }

  const { val, left, right } = node
  curmax = Math.max(curmax, val)
  curmin = Math.min(curmin, val)

  const l = travel(left, curmax, curmin)
  const r = travel(right, curmax, curmin)
  return Math.max(l, r)
}

function maxAncestorDiff (root: TreeNode | null): number {
  return root != null ? travel(root, root.val, root.val) : 0
}

async function main (): Promise<void> {
  const inputs: Array<TreeNode | null> = [
    newnode(8,
      newnode(3,
        newval(1),
        newnode(6,
          newval(4),
          newval(7)
        )
      ),
      newright(10,
        newleft(14,
          newval(13)
        )
      )
    ),
    newright(1,
      newright(2,
        newleft(0,
          newval(3)
        )
      )
    )
  ]

  for (const root of inputs) {
    const result = maxAncestorDiff(root)
    console.log(result)
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})

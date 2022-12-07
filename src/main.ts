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

function newleft (val: number, left: TreeNode): TreeNode {
  return new TreeNode(val, left)
}

function newright (val: number, right: TreeNode): TreeNode {
  return new TreeNode(val, undefined, right)
}

function newval (val: number): TreeNode {
  return new TreeNode(val)
}

function rangeSumBST (root: TreeNode | null, low: number, high: number): number {
  let result = 0

  const stack = [root]
  while (stack.length > 0) {
    const top = stack.pop()
    if (top == null) {
      continue
    }

    const { val, left, right } = top
    if ((low <= val) && (val <= high)) {
      result += val
    }

    if (low < val) {
      stack.push(left)
    }

    if (val < high) {
      stack.push(right)
    }
  }

  return result
}

interface Input {
  readonly root: TreeNode | null
  readonly low: number
  readonly high: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      low: 7,
      high: 15,
      root: newnode(10,
        newnode(5, newval(3), newval(7)),
        newright(15, newval(18))
      )
    },
    {
      low: 6,
      high: 10,
      root: newnode(10, newnode(5, newleft(3, newval(1)), newleft(7, newval(6))), newnode(5, newval(13), newval(18)))
    }
  ]

  for (const { low, high, root } of inputs) {
    const result = rangeSumBST(root, low, high)
    console.log(result)
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})

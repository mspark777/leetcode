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

function newnode (val: number, left?: TreeNode, right?: TreeNode): TreeNode {
  return new TreeNode(val, left, right)
}

function newleft (val: number, left?: TreeNode): TreeNode {
  return newnode(val, left)
}

function newval (val: number): TreeNode {
  return newnode(val)
}

interface StackNode {
  readonly path: number[]
  readonly node: TreeNode
  readonly sum: number
}

function pathSum (root: TreeNode | null, targetSum: number): number[][] {
  if (root == null) {
    return []
  }

  const result: number[][] = []
  const stack: StackNode[] = [{ path: [], node: root, sum: 0 }]
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { path, sum, node: { val, left, right } } = top
    const newsum = val + sum
    let isLeft = true
    if (left != null) {
      isLeft = false
      stack.push({
        path: [...path, val],
        node: left,
        sum: newsum
      })
    }

    if (right != null) {
      isLeft = false
      stack.push({
        path: [...path, val],
        node: right,
        sum: newsum
      })
    }

    if (!isLeft) {
      continue
    }

    if (newsum === targetSum) {
      result.push([...path, val])
    }
  }

  return result
}

interface Input {
  readonly root: TreeNode | null
  readonly targetSum: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: newnode(5, newleft(4, newnode(11, newval(7), newval(2))), newnode(8, newval(13), newnode(4, newval(5), newval(1)))),
      targetSum: 22
    },
    {
      root: newnode(1, newval(2), newval(3)),
      targetSum: 5
    },
    {
      root: newleft(1, newval(2)),
      targetSum: 0
    }
  ]

  for (const { root, targetSum } of inputs) {
    const result = pathSum(root, targetSum)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

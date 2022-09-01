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

interface StackNode {
  readonly node: TreeNode
  readonly max: number
}

function goodNodes (root: TreeNode | null): number {
  if (root == null) {
    return 0
  }

  let result = 0
  const stack: StackNode[] = [{ node: root, max: root.val }]
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { left, right, val } = top.node
    const max = Math.max(top.max, val)
    if (val === max) {
      result += 1
    }

    if (left != null) {
      stack.push({ node: left, max })
    }

    if (right != null) {
      stack.push({ node: right, max })
    }
  }

  return result
}

interface Input {
  readonly root: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: new TreeNode(3,
        new TreeNode(1, new TreeNode(3)),
        new TreeNode(4, new TreeNode(1), new TreeNode(5))
      )
    },
    {
      root: new TreeNode(3, new TreeNode(3, new TreeNode(4), new TreeNode(2)))
    },
    {
      root: new TreeNode(1)
    }
  ]

  for (const { root } of inputs) {
    const result = goodNodes(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

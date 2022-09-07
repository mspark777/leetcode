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

function newval (val: number): TreeNode {
  return newnode(val, null, null)
}

function newleft (val: number, left: TreeNode | null): TreeNode {
  return newnode(val, left, null)
}

function newright (val: number, right: TreeNode | null): TreeNode {
  return newnode(val, null, right)
}

function tree2str (root: TreeNode | null): string {
  if (root == null) {
    return ''
  }

  const stack: TreeNode[] = [root]
  const visiteds = new Set<TreeNode>()
  const result: string[] = []
  for (let node = stack.at(-1); node != null; node = stack.at(-1)) {
    if (visiteds.has(node)) {
      stack.pop()
      result.push(')')
      continue
    }

    visiteds.add(node)
    const { left, right, val } = node
    result.push('(', val.toString())
    if ((left == null) && (right != null)) {
      result.push('()')
    }

    if (right != null) {
      stack.push(right)
    }

    if (left != null) {
      stack.push(left)
    }
  }

  return result.slice(1, -1).join('')
}

interface Input {
  readonly root: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: newnode(1,
        newleft(2, newval(4)),
        newval(3)
      )
    },
    {
      root: newnode(1,
        newright(2, newval(4)),
        newval(3)
      )
    },
    {
      root: newleft(-1,
        newleft(-2, newleft(-3, newval(-4)))
      )
    }
  ]

  for (const { root } of inputs) {
    const result = tree2str(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

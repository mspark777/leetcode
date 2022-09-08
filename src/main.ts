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

function inorderTraversal (root: TreeNode | null): number[] {
  const stack: TreeNode[] = []
  const result: number[] = []
  let top = root
  while ((top != null) || (stack.length > 0)) {
    while (top != null) {
      stack.push(top)
      top = top.left
    }

    top = stack.pop() as TreeNode
    result.push(top.val)
    top = top.right
  }

  return result
}

interface Input {
  readonly root: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: newright(1, newleft(2, newval(3)))
    },
    {
      root: null
    },
    {
      root: newval(1)
    }
  ]

  for (const { root } of inputs) {
    const result = inorderTraversal(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

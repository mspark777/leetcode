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

function dfs (stack: number[], node: TreeNode | null): void {
  if (node == null) {
    return
  }

  const { val, left, right } = node
  if ((left == null) && (right == null)) {
    stack.push(val)
  }

  dfs(stack, left)
  dfs(stack, right)
}

function leafSimilar (root1: TreeNode | null, root2: TreeNode | null): boolean {
  const stack1 = new Array<number>()
  const stack2 = new Array<number>()
  dfs(stack1, root1)
  dfs(stack2, root2)

  if (stack1.length !== stack2.length) {
    return false
  }

  for (let i = 0; i < stack1.length; i += 1) {
    if (stack1[i] !== stack2[i]) {
      return false
    }
  }

  return true
}

interface Input {
  readonly root1: TreeNode | null
  readonly root2: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root1: newnode(3,
        newnode(5,
          newval(6),
          newnode(2,
            newval(7),
            newval(4)
          )
        ),
        newnode(1,
          newval(9),
          newval(8)
        )
      ),
      root2: newnode(3,
        newnode(5, newval(6), newval(7)),
        newnode(1,
          newval(4),
          newnode(2,
            newval(9),
            newval(8)
          )
        )
      )
    },
    {
      root1: newnode(1, newval(2), newval(3)),
      root2: newnode(1, newval(3), newval(2))
    }
  ]

  for (const { root1, root2 } of inputs) {
    const result = leafSimilar(root1, root2)
    console.log(result)
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})

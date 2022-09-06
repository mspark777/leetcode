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

function preorder (node: TreeNode | null): void {
  if (node != null) {
    preorder(node.left)
    process.stdout.write(`${node.val} `)
    preorder(node.right)
  }
}

function containsOne (node: TreeNode | null): boolean {
  if (node == null) {
    return false
  }

  const leftContained = containsOne(node.left)
  if (!leftContained) {
    node.left = null
  }

  const rightContained = containsOne(node.right)
  if (!rightContained) {
    node.right = null
  }

  return node.val === 1 || leftContained || rightContained
}

function pruneTree (root: TreeNode | null): TreeNode | null {
  return containsOne(root) ? root : null
}

interface Input {
  readonly root: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: newright(1,
        newnode(0,
          newval(0),
          newval(1)
        )
      )
    },
    {
      root: newnode(1,
        newnode(0,
          newval(0),
          newval(0)
        ),
        newnode(1,
          newval(0),
          newval(1)
        )
      )
    },
    {
      root: newnode(1,
        newnode(1,
          newleft(1,
            newval(0)
          ),
          newval(1)
        ),
        newnode(0,
          newval(0),
          newval(1)
        )
      )
    }
  ]

  for (const { root } of inputs) {
    const result = pruneTree(root)
    preorder(result)
    console.log()
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

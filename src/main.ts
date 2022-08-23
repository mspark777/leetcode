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

function invertTree (root: TreeNode | null): TreeNode | null {
  const stack: Array<TreeNode | null> = [root]
  while (stack.length > 0) {
    const node = stack.pop()
    if (node == null) {
      continue
    }

    const left = node.left
    const right = node.right
    node.right = left
    node.left = right
    stack.push(left)
    stack.push(right)
  }

  return root
}

interface Input {
  readonly root: TreeNode | null
}

function travelInorder (node: TreeNode | null): void {
  if (node != null) {
    travelInorder(node.left)
    process.stdout.write(`${node.val as number} `)
    travelInorder(node.right)
  }
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: new TreeNode(4,
        new TreeNode(2,
          new TreeNode(1),
          new TreeNode(3)
        ),
        new TreeNode(7,
          new TreeNode(6),
          new TreeNode(9)
        )
      )
    },
    {
      root: new TreeNode(2, new TreeNode(1), new TreeNode(3))
    },
    {
      root: null
    }
  ]

  for (const input of inputs) {
    const root = input.root
    const result = invertTree(root)
    travelInorder(result)
    console.log()
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

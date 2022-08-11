class TreeNode {
  val: number
  // eslint-disable-next-line no-use-before-define
  left: TreeNode | null
  // eslint-disable-next-line no-use-before-define
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function isValidBST (root: TreeNode | null): boolean {
  if (!root) {
    return true
  }

  let pre: TreeNode | null = null
  const stack: TreeNode[] = []
  while (root || (stack.length > 0)) {
    while (root) {
      stack.push(root)
      root = root.left
    }

    root = stack.pop() as TreeNode
    if (pre && (root.val <= pre.val)) {
      return false
    }
    pre = root
    root = root.right
  }

  return true
}

interface Input {
  root: TreeNode | null
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: new TreeNode(2,
        new TreeNode(1),
        new TreeNode(3)
      )
    },
    {
      root: new TreeNode(4,
        new TreeNode(1),
        new TreeNode(4,
          new TreeNode(3),
          new TreeNode(6)
        )
      )
    }
  ]

  for (const { root } of inputs) {
    const result = isValidBST(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

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

function minDepth (root: TreeNode | null): number {
  if (!root) {
    return 0
  } else if (root.left && root.right) {
    return Math.min(minDepth(root.left), minDepth(root.right)) + 1
  } else {
    return Math.max(minDepth(root.left), minDepth(root.right)) + 1
  }
}

async function main (): Promise<void> {
  const inputs = [
    new TreeNode(3,
      new TreeNode(9),
      new TreeNode(20,
        new TreeNode(15),
        new TreeNode(7)
      )
    ),
    new TreeNode(2,
      null,
      new TreeNode(3,
        null,
        new TreeNode(4,
          null,
          new TreeNode(5,
            null,
            new TreeNode(6)
          )
        )
      )
    )
  ]

  for (const input of inputs) {
    const result = minDepth(input)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

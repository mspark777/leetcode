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

function averageOfLevels (root: TreeNode | null): number[] {
  const result: number[] = []
  if (root == null) {
    return result
  }

  let queue: TreeNode[] = [root]
  while (queue.length > 0) {
    const size = queue.length
    let total = 0n
    for (let i = 0; i < size; i += 1) {
      const { val, left, right } = queue[i]
      total += BigInt(val)

      if (left != null) {
        queue.push(left)
      }

      if (right != null) {
        queue.push(right)
      }
    }

    queue = queue.slice(size)
    const sizen = BigInt(size)
    const div = total / sizen
    const rem = total % sizen
    const avg = Number(div) + (Number(rem) / size)
    result.push(Number(avg.toFixed(5)))
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
        new TreeNode(9),
        new TreeNode(20, new TreeNode(15), new TreeNode(7))
      )
    },
    {
      root: new TreeNode(3,
        new TreeNode(9,
          new TreeNode(15),
          new TreeNode(7)
        ),
        new TreeNode(20)
      )
    }
  ]

  for (const { root } of inputs) {
    const result = averageOfLevels(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

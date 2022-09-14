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

function newright (val: number, right: TreeNode): TreeNode {
  return newnode(val, null, right)
}

function newval (val: number): TreeNode {
  return newnode(val, null, null)
}

interface StackNode {
  readonly node: TreeNode | null
  readonly path: bigint
}

function pseudoPalindromicPaths (root: TreeNode | null): number {
  let result = 0
  const stack: StackNode[] = [{ node: root, path: 0n }]
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { node, path } = top

    if (node == null) {
      continue
    }

    const { left, right, val } = node
    const newPath = path ^ (1n << BigInt(val))
    if ((left == null) && (right == null)) {
      if ((newPath & (newPath - 1n)) === 0n) {
        result += 1
      }
    } else {
      stack.push({ node: left, path: newPath })
      stack.push({ node: right, path: newPath })
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
      root: newnode(2, newnode(3, newval(3), newval(1)), newright(1, newval(1)))
    },
    {
      root: newnode(2, newnode(1, newval(1), newright(3, newval(1))), newval(1))
    },
    {
      root: newval(9)
    }
  ]

  for (const { root } of inputs) {
    const result = pseudoPalindromicPaths(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

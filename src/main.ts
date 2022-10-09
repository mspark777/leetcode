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

function newnode (val: number, left?: TreeNode | null, right?: TreeNode | null): TreeNode {
  return new TreeNode(val, left, right)
}

function newright (val: number, right?: TreeNode | null): TreeNode {
  return newnode(val, null, right)
}

function newval (val: number): TreeNode {
  return newnode(val)
}

function findTarget (root: TreeNode | null, k: number): boolean {
  const stack: Array<TreeNode | null> = [root]
  const memo = new Set<number>()

  while (stack.length > 0) {
    const top = stack.pop()
    if (top == null) {
      continue
    }

    const { left, right, val } = top
    const target = k - val
    if (memo.has(target)) {
      return true
    }

    memo.add(val)
    stack.push(left, right)
  }

  return false
}

interface Input {
  readonly root: TreeNode | null
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
      k: 9
    },
    {
      root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
      k: 28
    }
  ]

  for (const { root, k } of inputs) {
    const result = findTarget(root, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

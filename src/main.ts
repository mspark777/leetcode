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

function newleft (val: number, left?: TreeNode | null): TreeNode {
  return newnode(val, left)
}

function newright (val: number, right?: TreeNode | null): TreeNode {
  return newnode(val, undefined, right)
}

function newval (val: number): TreeNode {
  return newnode(val)
}

interface StackNode {
  readonly node: TreeNode
  readonly target: number
}

function hasPathSum (root: TreeNode | null, targetSum: number): boolean {
  if (root == null) {
    return false
  }

  const stack: StackNode[] = [{ node: root, target: targetSum }]
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { node, target } = top
    const { val, left, right } = node
    const newTarget = target - val
    let isleaf = true
    if (left != null) {
      isleaf = false
      stack.push({
        node: left,
        target: newTarget
      })
    }

    if (right != null) {
      isleaf = false
      stack.push({
        node: right,
        target: newTarget
      })
    }

    if (isleaf && (newTarget === 0)) {
      return true
    }
  }

  return false
}

interface Input {
  readonly root: TreeNode | null
  readonly targetSum: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: newnode(5, newleft(4, newnode(11, newval(7), newval(2))), newnode(8, newval(13), newright(4, newval(1)))),
      targetSum: 22
    },
    {
      root: newnode(5, newval(2), newval(3)),
      targetSum: 5
    },
    {
      root: null,
      targetSum: 0
    }
  ]

  for (const { root, targetSum } of inputs) {
    const result = hasPathSum(root, targetSum)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

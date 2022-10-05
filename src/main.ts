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

function treetoarr (node: TreeNode | null): number[] {
  const nums: number[] = []
  const travel = (n: TreeNode | null): void => {
    if (n != null) {
      nums.push(n.val)
      travel(n.left)
      travel(n.right)
    }
  }
  travel(node)

  return nums
}

interface StackNode {
  readonly node: TreeNode
  readonly pos: number
}

function addOneRow (root: TreeNode | null, val: number, depth: number): TreeNode | null {
  if (root == null) {
    return null
  }

  if (depth === 1) {
    return new TreeNode(val, root)
  }

  const stack: StackNode[] = [{
    node: root,
    pos: 2
  }]

  const targets: TreeNode[] = []
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { pos, node } = top
    if (pos > depth) {
      continue
    }

    if (pos === depth) {
      targets.push(node)
      continue
    }

    const { left, right } = node
    if (left != null) {
      stack.push({
        node: left,
        pos: pos + 1
      })
    }

    if (right != null) {
      stack.push({
        node: right,
        pos: pos + 1
      })
    }
  }

  for (const target of targets) {
    target.left = new TreeNode(val, target.left)
    target.right = new TreeNode(val, undefined, target.right)
  }

  return root
}

interface Input {
  readonly root: TreeNode | null
  readonly val: number
  readonly depth: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
      val: 1,
      depth: 2
    },
    {
      root: newleft(4, newnode(2, newval(3), newval(1))),
      val: 1,
      depth: 3
    },
    {
      root: newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
      val: 1,
      depth: 1
    }
  ]

  for (const { root, val, depth } of inputs) {
    const result = addOneRow(root, val, depth)
    console.log(treetoarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

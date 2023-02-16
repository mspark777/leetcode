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

function newright (val: number, right: TreeNode): TreeNode {
  return new TreeNode(val, undefined, right)
}

function newval (val: number): TreeNode {
  return new TreeNode(val)
}

function travel (node: TreeNode | null, depth: number, ref: number[]): void {
  if (node != null) {
    const d = depth + 1
    travel(node.left, d, ref)
    travel(node.right, d, ref)
  } else {
    ref[0] = Math.max(ref[0], depth)
  }
}

function maxDepth (root: TreeNode | null): number {
  const result = [0]
  travel(root, 0, result)

  return result[0]
}

async function main (): Promise<void> {
  const inputs: TreeNode[] = [
    newnode(3, newval(9), newnode(20, newval(15), newval(7))),
    newright(1, newval(2))
  ]

  for (const root of inputs) {
    const result = maxDepth(root)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })

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

function newleft (val: number, left: TreeNode): TreeNode {
  return new TreeNode(val, left)
}

function newright (val: number, right: TreeNode): TreeNode {
  return new TreeNode(val, undefined, right)
}

function newval (val: number): TreeNode {
  return new TreeNode(val)
}

interface Data {
  prevNode?: TreeNode
  minDistance: number
}

function travel (root: TreeNode | null, data: Data): number {
  if (root == null) {
    return data.minDistance
  }

  travel(root.left, data)

  if (data.prevNode != null) {
    data.minDistance = Math.min(data.minDistance, root.val - data.prevNode.val)
  }
  data.prevNode = root
  travel(root.right, data)
  return data.minDistance
}

function minDiffInBST (root: TreeNode | null): number {
  return travel(root, { minDistance: Number.MAX_SAFE_INTEGER })
}

async function main (): Promise<void> {
  const inputs: TreeNode[] = [
    newnode(4, newnode(2, newval(1), newval(3)), newval(6)),
    newnode(1, newval(0), newnode(48, newval(12), newval(49))),
    newright(27, newright(34, newleft(58, newleft(50, newval(44)))))
  ]

  for (const root of inputs) {
    const result = minDiffInBST(root)
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

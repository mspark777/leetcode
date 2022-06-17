import { isEmpty } from 'lodash'

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

enum Status {
  LEAF, CAMERA, NOCAMERA
}

class DFS {
  depth: number
  constructor () {
    this.depth = 0
  }

  travel (node: TreeNode | null): Status {
    if (isEmpty(node)) {
      return Status.NOCAMERA
    }

    const left = this.travel(node!.left)
    const right = this.travel(node!.right)
    const statuses = [left, right]

    if (statuses.includes(Status.LEAF)) {
      this.depth += 1
      return Status.CAMERA
    } else if (statuses.includes(Status.CAMERA)) {
      return Status.NOCAMERA
    }

    return Status.LEAF
  }
}

function minCameraCover (root: TreeNode | null): number {
  const dfs = new DFS()
  const status = dfs.travel(root)
  const depth = dfs.depth
  return status === Status.LEAF ? depth + 1 : depth
}

function arrToTree (arr: (number | null)[], i: number): TreeNode | null {
  if (i >= arr.length) {
    return null
  }

  const val = arr[i]
  if (val === null) {
    return null
  }

  const node = new TreeNode(val)
  node.left = arrToTree(arr, i * 2 + 1)
  node.right = arrToTree(arr, (i + 1) * 2)
  return node
}

async function main (): Promise<void> {
  const inputs = [
    [0, 0, null, 0, 0],
    [0, 0, null, 0, null, 0, null, null, 0]
  ]

  for (const input of inputs) {
    const result = minCameraCover(arrToTree(input, 0))
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

import lodash from 'lodash'
const { isEmpty } = lodash

class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

const LEAF = 1
const CAMERA = 2
const NOCAMERA = 3

class DFS {
  constructor () {
    this.depth = 0
  }

  travel (node) {
    if (isEmpty(node)) {
      return NOCAMERA
    }

    const left = this.travel(node.left)
    const right = this.travel(node.right)
    const statuses = [left, right]

    if (statuses.includes(LEAF)) {
      this.depth += 1
      return CAMERA
    } else if (statuses.includes(CAMERA)) {
      return NOCAMERA
    }

    return LEAF
  }
}

function minCameraCover (root) {
  const dfs = new DFS()
  const status = dfs.travel(root)
  const depth = dfs.depth
  return status === LEAF ? depth + 1 : depth
}

function arrToTree (arr, i) {
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

async function main () {
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

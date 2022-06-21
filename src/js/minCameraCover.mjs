import { isEmpty } from 'lodash'

// eslint-disable-next-line @typescript-eslint/no-unused-vars
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

/**
 * @param {TreeNode} root
 * @return {number}
 */
export function minCameraCover (root) {
  const dfs = new DFS()
  const status = dfs.travel(root)
  const depth = dfs.depth
  return status === LEAF ? depth + 1 : depth
}

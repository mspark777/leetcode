export class TreeNode {
  val: number
  // eslint-disable-next-line no-use-before-define
  left: TreeNode | null
  // eslint-disable-next-line no-use-before-define
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val ?? 0
    this.left = left ?? null
    this.right = right ?? null
  }
}

// https://leetcode.com/problems/invert-binary-tree/discuss/62896/Just-wonder-how-to-write-custom-testcase.-Any-examples/499471
export function createTreeFromArray (arr: (number | null)[]) {
  let root = null
  const q = []
  let i = 0
  const t = arr[i] == null ? null : new TreeNode(arr[i] as number)
  root = t
  q.push(root)
  i++
  while (q.length && i < arr.length) {
    const t1 = q.shift()
    if (t1 != null) {
      t1.left = arr[i] == null ? null : new TreeNode(arr[i] as number)
      q.push(t1.left)
      i++
      if (i >= arr.length) {
        break
      }
      t1.right = arr[i] == null ? null : new TreeNode(arr[i] as number)
      q.push(t1.right)
      i++
    }
  }
  return root
}

export function inorderTraversal (root: TreeNode | null): number[] {
  const results: number[] = []
  const dfs = (r: TreeNode | null) => {
    if (r !== null) {
      if (r.left) dfs(r.left)
      results.push(r.val)
      if (r.right) dfs(r.right)
    }
  }
  dfs(root)
  return results
}

export function rightSideView (root: TreeNode | null): number[] {
  if (!root) {
    return []
  }

  const result: number[] = []
  const q: TreeNode[] = [root]
  while (q.length > 0) {
    const n = q.length
    for (let i = 1; i <= n; i += 1) {
      const node = q.shift() as TreeNode
      if (i === n) {
        result.push(node.val)
      }

      if (node.left) {
        q.push(node.left)
      }

      if (node.right) {
        q.push(node.right)
      }
    }
  }

  return result
}

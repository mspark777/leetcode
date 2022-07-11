export class TreeNode {
  constructor (val, left, right) {
    this.val = val ?? 0
    this.left = left ?? null
    this.right = right ?? null
  }
}

// https://leetcode.com/problems/invert-binary-tree/discuss/62896/Just-wonder-how-to-write-custom-testcase.-Any-examples/499471
export function createTreeFromArray (arr) {
  let root = null
  const q = []
  let i = 0
  const t = arr[i] == null ? null : new TreeNode(arr[i])
  root = t
  q.push(root)
  i++
  while (q.length && i < arr.length) {
    const t1 = q.shift()
    if (t1 != null) {
      t1.left = arr[i] == null ? null : new TreeNode(arr[i])
      q.push(t1.left)
      i++
      if (i >= arr.length) {
        break
      }
      t1.right = arr[i] == null ? null : new TreeNode(arr[i])
      q.push(t1.right)
      i++
    }
  }
  return root
}

export function inorderTraversal (root) {
  const results = []
  const dfs = r => {
    if (r !== null) {
      if (r.left) dfs(r.left)
      results.push(r.val)
      if (r.right) dfs(r.right)
    }
  }
  dfs(root)
  return results
}

export function rightSideView (root) {
  if (!root) {
    return []
  }

  const result = []
  const q = [root]
  while (q.length > 0) {
    const n = q.length
    for (let i = 1; i <= n; i += 1) {
      const node = q.shift()
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

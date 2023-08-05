import '@total-typescript/ts-reset'

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

function clone (node: TreeNode | null, offset: number): TreeNode | null {
  if (node == null) {
    return null
  }

  const cloned = new TreeNode(node.val + offset)
  cloned.left = clone(node.left, offset)
  cloned.right = clone(node.right, offset)
  return cloned
}

function generateTrees (n: number): Array<TreeNode | null> {
  const dp: Array<Array<TreeNode | null>> = []
  for (let i = 0; i <= n; i += 1) {
    dp.push([])
  }

  dp[0]?.push(null)

  for (let numNodes = 1; numNodes <= n; numNodes += 1) {
    for (let i = 1; i <= numNodes; i += 1) {
      const j = numNodes - i
      for (const left of dp[i - 1] ?? []) {
        for (const right of dp[j] ?? []) {
          const root = new TreeNode(i, left, clone(right, i))
          dp[numNodes]?.push(root)
        }
      }
    }
  }

  return dp[n] as Array<TreeNode | null>
}

function toarr (node: TreeNode | null, arr: Array<number | null>): Array<number | null> {
  arr.push(node?.val ?? null)
  if (node != null) {
    toarr(node.left, arr)
    toarr(node.right, arr)
  }

  return arr
}

function main (): void {
  const inputs: number[] = [
    3, 1
  ]

  for (const n of inputs) {
    const result = generateTrees(n)
    for (const r of result) {
      const nums = toarr(r, [])
      console.log(nums)
    }
  }
}
main()

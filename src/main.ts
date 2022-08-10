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

function travel (nums: number[], l: number, r: number): TreeNode | null {
  if (l > r) {
    return null
  }

  if (l === r) {
    return new TreeNode(nums[l])
  }

  const mid = Math.trunc((l + r) / 2)
  return new TreeNode(
    nums[mid],
    travel(nums, l, mid - 1),
    travel(nums, mid + 1, r)
  )
}

function sortedArrayToBST (nums: number[]): TreeNode | null {
  return travel(nums, 0, nums.length - 1)
}

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [-10, -3, 0, 5, 9]
    },
    {
      nums: [1, 3]
    }
  ]

  for (const { nums } of inputs) {
    const result = sortedArrayToBST(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

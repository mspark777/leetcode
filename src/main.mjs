class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function travel (nums, l, r) {
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

/**
 * @param {number[]} nums
 * @return {TreeNode}
 */
function sortedArrayToBST (nums) {
  return travel(nums, 0, nums.length - 1)
}

async function main () {
  const inputs = [
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

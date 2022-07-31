function buildTree (nums: number[]): number[] {
  const tree = new Array(nums.length * 2).fill(0)
  for (let i = nums.length, j = 0; i < tree.length; i += 1, j += 1) {
    tree[i] = nums[j]
  }

  for (let i = nums.length - 1; i > 0; i -= 1) {
    tree[i] = tree[i * 2] + tree[i * 2 + 1]
  }

  return tree
}

export class NumArray {
  readonly tree: number[]
  readonly numCount: number
  constructor (nums: number[]) {
    this.numCount = nums.length
    this.tree = buildTree(nums)
  }

  update (index: number, val: number): void {
    index += this.numCount
    const tree = this.tree

    tree[index] = val
    while (index > 0) {
      let left = index
      let right = index
      if (index % 2 === 0) {
        right = index + 1
      } else {
        left = index - 1
      }

      index = Math.trunc(index / 2)
      tree[index] = tree[left] + tree[right]
    }
  }

  sumRange (left: number, right: number): number {
    left += this.numCount
    right += this.numCount

    const tree = this.tree
    let sum = 0
    while (left <= right) {
      if ((left % 2) === 1) {
        sum += tree[left]
        left++
      }

      if ((right % 2) === 0) {
        sum += tree[right]
        right--
      }

      left = Math.trunc(left / 2)
      right = Math.trunc(right / 2)
    }
    return sum
  }
}

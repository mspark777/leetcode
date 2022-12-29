class NumArray {
  private readonly sums: number[]
  public constructor(nums: number[]) {
    this.sums = new Array(nums.length + 1).fill(0)
    nums.reduce((acc, cur, i) => {
      const sum = acc + cur
      this.sums[i + 1] = sum
      return sum
    }, 0)
  }

  public sumRange(left: number, right: number): number {
    const {sums} = this
    return sums[right + 1] - sums[left]
  }
}

async function main(): Promise<void> {
  const obj = new NumArray([-2, 0, 3, -5, 2, -1])
  console.log(obj.sumRange(0, 2))
  console.log(obj.sumRange(2, 5))
  console.log(obj.sumRange(0, 5))
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

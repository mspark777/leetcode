class RandomizedSet {
  private readonly nums: number[]
  private readonly indexes: Map<number, number>
  constructor () {
    this.nums = []
    this.indexes = new Map()
  }

  public insert (val: number): boolean {
    const { nums, indexes } = this

    if (indexes.has(val)) {
      return false
    }

    indexes.set(val, nums.length)
    nums.push(val)
    return true
  }

  public remove (val: number): boolean {
    const { nums, indexes } = this
    if (!indexes.has(val)) {
      return false
    }

    const pos = indexes.get(val) as number
    this.swap(pos)
    nums.pop()
    if (nums.length > 0) {
      indexes.set(nums[pos], pos)
    }

    indexes.delete(val)
    return true
  }

  public getRandom (): number {
    const { nums } = this
    const count = nums.length
    const random = Math.round(Math.random() * count * 10)
    const index = random % count
    return nums[index]
  }

  private swap (pos: number): void {
    const { nums } = this
    const last = nums.length - 1
    if (last < 0) {
      return
    }

    const temp = nums[pos]
    nums[pos] = nums[last]
    nums[last] = temp
  }
}

async function main (): Promise<void> {
  const obj = new RandomizedSet()
  console.log(obj.insert(1))
  console.log(obj.remove(2))
  console.log(obj.insert(2))
  console.log(obj.getRandom())
  console.log(obj.remove(1))
  console.log(obj.insert(2))
  console.log(obj.getRandom())
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

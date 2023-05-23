import '@total-typescript/ts-reset'

class KthLargest {
  private readonly k: number
  private readonly nums: number[]
  public constructor (k: number, nums: number[]) {
    this.k = k - 1
    this.nums = nums
    this.nums.sort((a, b) => b - a)
  }

  public add (val: number): number {
    let i = 0
    while (i < this.nums.length) {
      if (val > this.nums[i]) {
        break
      } else {
        i += 1
      }
    }

    if (i < this.nums.length) {
      this.nums.splice(i, 0, val)
    } else {
      this.nums.push(val)
    }

    return this.nums[this.k]
  }
}

async function main (): Promise<void> {
  const kthLargest = new KthLargest(3, [4, 5, 8, 2])
  console.log(kthLargest.add(3))
  console.log(kthLargest.add(5))
  console.log(kthLargest.add(10))
  console.log(kthLargest.add(9))
  console.log(kthLargest.add(4))
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })

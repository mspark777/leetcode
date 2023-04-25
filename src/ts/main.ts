class SmallestInfiniteSet {
  private current: number
  private readonly set: Set<number>
  public constructor () {
    this.current = 1
    this.set = new Set()
  }

  public popSmallest (): number {
    if (this.set.size < 1) {
      const result = this.current
      this.current += 1

      return result
    }

    let result = Number.MAX_SAFE_INTEGER
    for (const num of this.set) {
      result = Math.min(result, num)
    }

    this.set.delete(result)
    return result
  }

  public addBack (num: number): void {
    if (this.current <= num) {
      return
    } else if (this.set.has(num)) {
      return
    }

    this.set.add(num)
  }
}

async function main (): Promise<void> {
  const smallestInfiniteSet = new SmallestInfiniteSet()
  smallestInfiniteSet.addBack(2)
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
  smallestInfiniteSet.addBack(1)
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })

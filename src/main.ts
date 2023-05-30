import '@total-typescript/ts-reset'

class MyHashSet {
  private readonly size: number
  private readonly nums: Array<number | null>
  public constructor () {
    this.size = 1000000
    this.nums = new Array(this.size).fill(null)
  }

  public add (key: number): void {
    this.nums[key] = key
  }

  public remove (key: number): void {
    this.nums[key] = null
  }

  public contains (key: number): boolean {
    return this.nums[key] != null
  }
}

async function main (): Promise<void> {
  const myHashSet = new MyHashSet()
  myHashSet.add(1)
  myHashSet.add(2)
  console.log(myHashSet.contains(1))
  console.log(myHashSet.contains(3))
  myHashSet.add(2)
  console.log(myHashSet.contains(2))
  myHashSet.remove(2)
  console.log(myHashSet.contains(2))
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })

class Baskets {
  private readonly map: Map<number, number>
  public constructor () {
    this.map = new Map()
  }

  public get (key: number): number {
    return this.map.get(key) ?? 0
  }

  public set (key: number, value: number): void {
    this.map.set(key, value)
  }

  public delete (key: number): void {
    this.map.delete(key)
  }

  public increase (key: number): void {
    this.set(key, this.get(key) + 1)
  }

  public decrease (key: number): void {
    this.set(key, this.get(key) - 1)
  }

  public size (): number {
    return this.map.size
  }
}

function totalFruit (fruits: number[]): number {
  const baskets = new Baskets()
  let left = 0
  let result = 0

  for (let right = 0; right < fruits.length; right += 1) {
    const rfruit = fruits[right]
    baskets.increase(rfruit)

    while (baskets.size() > 2) {
      const lfruit = fruits[left]
      baskets.decrease(lfruit)
      if (baskets.get(lfruit) === 0) {
        baskets.delete(lfruit)
      }
      left += 1
    }

    result = Math.max(result, right - left + 1)
  }

  return result
}

async function main (): Promise<void> {
  const inputs = [
    [1, 2, 1],
    [0, 1, 2, 2],
    [1, 2, 3, 2, 2],
    [3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]
  ]

  for (const fruits of inputs) {
    const result = totalFruit(fruits)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })

class Helper {
  private readonly map: Map<number, number>
  private islands: number
  public constructor () {
    this.map = new Map()
    this.islands = 0
  }

  public find (x: number): number {
    const { map } = this
    if (!map.has(x)) {
      map.set(x, x)
      this.islands += 1
    }

    const p = map.get(x) as number
    if (x !== p) {
      map.set(x, this.find(p))
    }

    return map.get(x) as number
  }

  public uni (x: number, y: number): void {
    x = this.find(x)
    y = this.find(y)
    if (x !== y) {
      this.map.set(x, y)
      this.islands -= 1
    }
  }

  public getIsLands (): number {
    return this.islands
  }
}

function removeStones (stones: number[][]): number {
  const helper = new Helper()
  for (const [x, y] of stones) {
    helper.uni(x, ~y)
  }

  return stones.length - helper.getIsLands()
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]],
    [[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]],
    [[0, 0], [0, 1], [1, 0], [1, 1], [2, 1], [2, 2], [3, 2], [3, 3], [3, 4], [4, 3], [4, 4]]
  ]

  for (const stones of inputs) {
    const result = removeStones(stones)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

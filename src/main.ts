function findCheapestPrice (n: number, flights: number[][], src: number, dst: number, k: number): number {
  let dists = new Array<number>(n).fill(Number.MAX_SAFE_INTEGER)
  dists[src] = 0

  for (let i = 0; i <= k; i += 1) {
    const temp = [...dists]
    for (const [f, t, p] of flights) {
      if (dists[f] !== Number.MAX_SAFE_INTEGER) {
        temp[t] = Math.min(temp[t], dists[f] + p)
      }
    }
    dists = temp
  }

  const result = dists[dst]
  return result === Number.MAX_SAFE_INTEGER ? -1 : result
}

interface Input {
  readonly n: number
  readonly flights: number[][]
  readonly src: number
  readonly dst: number
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: 4, flights: [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]], src: 0, dst: 3, k: 1 },
    { n: 3, flights: [[0, 1, 100], [1, 2, 100], [0, 2, 500]], src: 0, dst: 2, k: 1 },
    { n: 3, flights: [[0, 1, 100], [1, 2, 100], [0, 2, 500]], src: 0, dst: 2, k: 0 }
  ]

  for (const { n, flights, src, dst, k } of inputs) {
    const result = findCheapestPrice(n, flights, src, dst, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

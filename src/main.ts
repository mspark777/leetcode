import '@total-typescript/ts-reset'

function maximalNetworkRank (n: number, roads: number[][]): number {
  const adjacents = new Map<number, Set<number>>()
  for (const road of roads) {
    const [a, b] = road as [number, number]
    const aSet = adjacents.get(a) ?? new Set()
    const bSet = adjacents.get(b) ?? new Set()

    aSet.add(b)
    bSet.add(a)
    adjacents.set(a, aSet)
    adjacents.set(b, bSet)
  }

  let result = 0
  for (let node0 = 0; node0 < n; node0 += 1) {
    const set0 = adjacents.get(node0) ?? new Set()
    const rank0 = set0.size ?? 0
    for (let node1 = node0 + 1; node1 < n; node1 += 1) {
      const rank1 = adjacents.get(node1)?.size ?? 0
      let rank = rank0 + rank1
      if (set0.has(node1)) {
        rank -= 1
      }

      result = Math.max(result, rank)
    }
  }

  return result
}

interface Input {
  readonly n: number
  readonly roads: number[][]
}

function main (): void {
  const inputs: Input[] = [
    { n: 4, roads: [[0, 1], [0, 3], [1, 2], [1, 3]] },
    { n: 5, roads: [[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]] },
    { n: 8, roads: [[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]] }
  ]

  for (const { n, roads } of inputs) {
    const result = maximalNetworkRank(n, roads)
    console.log(result)
  }
}
main()

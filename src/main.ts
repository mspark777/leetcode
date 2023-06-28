import '@total-typescript/ts-reset'

function maxProbability (n: number, edges: number[][], succProb: number[], start: number, end: number): number {
  const maxProbs = new Array<number>(n).fill(0)
  maxProbs[start] = 1

  for (let i = 0; i < n - 1; i += 1) {
    let breakable = true
    for (const [j, [u, v]] of edges.entries()) {
      const prob = succProb[j]
      const umax = maxProbs[u] * prob
      if (umax > maxProbs[v]) {
        maxProbs[v] = umax
        breakable = false
      }

      const vmax = maxProbs[v] * prob
      if (vmax > maxProbs[u]) {
        maxProbs[u] = vmax
        breakable = false
      }
    }

    if (breakable) {
      break
    }
  }

  return maxProbs[end]
}

interface Input {
  readonly n: number
  readonly edges: number[][]
  readonly succProb: number[]
  readonly start: number
  readonly end: number
}

function main (): void {
  const inputs: Input[] = [
    { n: 3, edges: [[0, 1], [1, 2], [0, 2]], succProb: [0.5, 0.5, 0.2], start: 0, end: 2 },
    { n: 3, edges: [[0, 1], [1, 2], [0, 2]], succProb: [0.5, 0.5, 0.3], start: 0, end: 2 },
    { n: 3, edges: [[0, 1]], succProb: [0.5], start: 0, end: 2 }
  ]

  for (const { n, edges, succProb, start, end } of inputs) {
    const result = maxProbability(n, edges, succProb, start, end)
    console.log(result)
  }
}
main()

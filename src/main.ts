import '@total-typescript/ts-reset'

function findSmallestSetOfVertices (n: number, edges: number[][]): number[] {
  const toEdges = new Array<boolean>(n).fill(false)
  for (const edge of edges) {
    const to = edge[1]
    toEdges[to] = true
  }

  const result: number[] = []
  for (const [i, isto] of toEdges.entries()) {
    if (!isto) {
      result.push(i)
    }
  }

  return result
}

interface Input {
  readonly n: number
  readonly edges: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: 6, edges: [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]] },
    { n: 5, edges: [[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]] }
  ]

  for (const { n, edges } of inputs) {
    const result = findSmallestSetOfVertices(n, edges)
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

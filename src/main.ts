class UnionFind {
  private readonly roots: number[]
  private readonly ranks: number[]
  public constructor (n: number) {
    this.roots = new Array(n).fill(0).map((_, i) => i)
    this.ranks = new Array(n).fill(1)
  }

  public find (x: number): number {
    const { roots } = this
    if (roots[x] !== x) {
      roots[x] = this.find(roots[x])
    }

    return roots[x]
  }

  public union (x: number, y: number): void {
    let rootx = this.find(x)
    let rooty = this.find(y)
    if (rootx !== rooty) {
      const { ranks } = this
      if (ranks[rootx] > ranks[rooty]) {
        const temp = rootx
        rootx = rooty
        rooty = temp
      }

      const { roots } = this
      roots[rootx] = rooty
      ranks[rooty] += ranks[rootx]
    }
  }
}

function validPath (n: number, edges: number[][], source: number, destination: number): boolean {
  const uf = new UnionFind(n)
  for (const [x, y] of edges) {
    uf.union(x, y)
  }

  return uf.find(source) === uf.find(destination)
}

interface Input {
  n: number
  edges: number[][]
  source: number
  destination: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 3,
      edges: [[0, 1], [1, 2], [2, 0]],
      source: 0,
      destination: 2
    },
    {
      n: 6,
      edges: [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]],
      source: 0,
      destination: 5
    }
  ]

  for (const { n, edges, source, destination } of inputs) {
    const result = validPath(n, edges, source, destination)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

class DFS {
  private readonly result: number[]
  private readonly counts: number[]
  private readonly graph: Array<Set<number>>
  private readonly N: number
  public constructor (N: number) {
    this.N = N
    this.graph = []
    this.counts = new Array(N).fill(1)
    this.result = new Array(N).fill(0)
  }

  public dfs (node: number, parent: number): void {
    const { result, counts } = this
    for (const child of this.graph[node]) {
      if (child !== parent) {
        this.dfs(child, node)
        counts[node] += counts[child]
        result[node] += result[child] + counts[child]
      }
    }
  }

  public dfs2 (node: number, parent: number): void {
    const { result, counts, N } = this
    for (const child of this.graph[node]) {
      if (child !== parent) {
        result[child] = result[node] - counts[child] + N - counts[child]
        this.dfs2(child, node)
      }
    }
  }

  public sumOfDistancesInTree (edges: number[][]): number[] {
    for (let i = 0; i < this.N; i += 1) {
      this.graph.push(new Set())
    }

    for (const edge of edges) {
      this.graph[edge[0]].add(edge[1])
      this.graph[edge[1]].add(edge[0])
    }

    this.dfs(0, -1)
    this.dfs2(0, -1)
    return this.result
  }
}

function sumOfDistancesInTree (n: number, edges: number[][]): number[] {
  const dfs = new DFS(n)
  return dfs.sumOfDistancesInTree(edges)
}

interface Input {
  readonly n: number
  readonly edges: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 6,
      edges: [[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]
    },
    {
      n: 1,
      edges: []
    },
    {
      n: 2,
      edges: [[1, 0]]
    }
  ]

  for (const { n, edges } of inputs) {
    const result = sumOfDistancesInTree(n, edges)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

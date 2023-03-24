class DFS {
  private count: number
  public constructor () {
    this.count = 0
  }

  public dfs (node: number, parent: number, adjs: Map<number, number[][]>): number {
    const edges = adjs.get(node)
    if (edges == null) {
      return this.count
    }

    for (const [child, sign] of edges) {
      if (child !== parent) {
        this.count += sign
        this.dfs(child, node, adjs)
      }
    }

    return this.count
  }
}

function minReorder (_n: number, connections: number[][]): number {
  const adjs = new Map<number, number[][]>()
  for (const [a, b] of connections) {
    const aedges = adjs.get(a) ?? []
    const bedges = adjs.get(b) ?? []
    aedges.push([b, 1])
    bedges.push([a, 0])

    adjs.set(a, aedges)
    adjs.set(b, bedges)
  }

  const dfs = new DFS()
  return dfs.dfs(0, -1, adjs)
}

interface Input {
  readonly n: number
  readonly connections: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: 6, connections: [[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]] },
    { n: 5, connections: [[1, 0], [1, 2], [3, 2], [3, 4]] },
    { n: 0, connections: [[1, 0], [2, 0]] }
  ]

  for (const { n, connections } of inputs) {
    const result = minReorder(n, connections)
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

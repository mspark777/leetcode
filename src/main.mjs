class DFS {
  /**
   * @param {number} N
   */
  constructor (N) {
    this.N = N
    this.graph = []
    this.counts = new Array(N).fill(1)
    this.result = new Array(N).fill(0)
  }

  /**
   * @param {number} node
   * @param {number} parent
   * @returns {undefined}
   */
  dfs (node, parent) {
    const { result, counts } = this
    for (const child of this.graph[node]) {
      if (child !== parent) {
        this.dfs(child, node)
        counts[node] += counts[child]
        result[node] += result[child] + counts[child]
      }
    }
  }

  /**
   * @param {number} node
   * @param {number} parent
   * @returns {undefined}
   */
  dfs2 (node, parent) {
    const { result, counts, N } = this
    for (const child of this.graph[node]) {
      if (child !== parent) {
        result[child] = result[node] - counts[child] + N - counts[child]
        this.dfs2(child, node)
      }
    }
  }

  /**
   * @param {number[][]} edges
   * @returns {number[]}
   */
  sumOfDistancesInTree (edges) {
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

/**
 * @param {number} n
 * @param {number[][]} edges
 * @returns {number[]}
 */
function sumOfDistancesInTree (n, edges) {
  const dfs = new DFS(n)
  return dfs.sumOfDistancesInTree(edges)
}

async function main () {
  const inputs = [
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

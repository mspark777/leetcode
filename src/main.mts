function dfs(graph: number[][], results: number[][], path: number[], cur: number): void {
  path.push(cur)
  if (cur === (graph.length - 1)) {
    results.push(path.slice())
  } else {
    for (const next of graph[cur]) {
      dfs(graph, results, path, next)
    }
  }

  path.pop()
}

function allPathsSourceTarget(graph: number[][]): number[][] {
  const results = new Array<number[]>()
  const path = new Array<number>()

  dfs(graph, results, path, 0)
  return results
}

async function main(): Promise<void> {
  const inputs: number[][][] = [
    [[1, 2], [3], [3], []],
    [[4, 3, 1], [3, 2, 4], [3], [4], []]
  ]

  for (const graph of inputs) {
    const result = allPathsSourceTarget(graph)
    console.log(result)
  }
}

await main()

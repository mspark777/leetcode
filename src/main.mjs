/**
 * @param {number[][]} graph
 * @param {number[][]} results
 * @param {number[]} path
 * @param {number} cur
 * @returns {undefined}
 */
function dfs(graph, results, path, cur) {
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

/**
 * @param {number[][]} graph
 * @returns {number[][]}
 */
function allPathsSourceTarget(graph) {
  const results = []
  const path = []

  dfs(graph, results, path, 0)
  return results
}

async function main() {
  const inputs = [
    [[1, 2], [3], [3], []],
    [[4, 3, 1], [3, 2, 4], [3], [4], []]
  ]

  for (const graph of inputs) {
    const result = allPathsSourceTarget(graph)
    console.log(result)
  }
}

await main()

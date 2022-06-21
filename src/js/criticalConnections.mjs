/**
 * @param {number} n
 * @param {number[][]} connections
 * @return {number[][]}
 */
export const criticalConnections = function (n, connections) {
  let step = 0
  const steps = new Array(n)
  const minSteps = new Array(n)
  const graph = new Array(n)
  for (let i = 0; i < n; i += 1) {
    graph[i] = []
  }

  for (const conn of connections) {
    const v = conn[0]
    const u = conn[1]
    graph[v].push(u)
    graph[u].push(v)
  }

  const result = []
  const dfs = (current, parent) => {
    step += 1
    steps[current] = step
    minSteps[current] = step

    for (const to of graph[current]) {
      if (!steps[to]) {
        dfs(to, current)
        minSteps[current] = Math.min(minSteps[current], minSteps[to])
      } else if (to !== parent) {
        minSteps[current] = Math.min(minSteps[current], minSteps[to])
      }

      if (minSteps[to] > steps[current]) {
        result.push([current, to])
      }
    }
  }

  dfs(0, -1)
  return result
}

/**
 * @param {number} node
 * @param {number} prevNode
 * @param {number[][]} adjMat
 * @param {boolean[]} hasApple
 * @returns {number}
 */
function dfs (node, prevNode, adjMat, hasApple) {
  let totalTime = 0
  let childTime = 0

  for (const child of adjMat[node]) {
    if (child === prevNode) {
      continue
    }

    childTime = dfs(child, node, adjMat, hasApple)
    if ((childTime > 0) || hasApple[child]) {
      totalTime += childTime + 2
    }
  }

  return totalTime
}

function minTime (n, edges, hasApple) {
  const adjMat = Array.from(new Array(n), () => [])
  for (const [l, r] of edges) {
    adjMat[l].push(r)
    adjMat[r].push(l)
  }

  return dfs(0, -1, adjMat, hasApple)
}

async function main () {
  const inputs = [
    { n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]], hasApple: [false, false, true, false, true, true, false] },
    { n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]], hasApple: [false, false, true, false, false, true, false] },
    { n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]], hasApple: [false, false, false, false, false, false, false] }
  ]

  for (const { n, edges, hasApple } of inputs) {
    const result = minTime(n, edges, hasApple)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

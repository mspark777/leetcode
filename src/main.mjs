/**
  * @param {number} n
  * @param {number[][]} edges
  * @returns {number[]}
  */
function findSmallestSetOfVertices (n, edges) {
  const toEdges = new Array(n).fill(false)
  for (const edge of edges) {
    const to = edge[1]
    toEdges[to] = true
  }

  const result = []
  for (const [i, isto] of toEdges.entries()) {
    if (!isto) {
      result.push(i)
    }
  }

  return result
}

async function main () {
  const inputs = [
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

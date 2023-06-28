/**
  * @param {number} n
  * @param {number[][]} edges
  * @param {number[]} succProb
  * @param {number} start
  * @param {number} end
  * @returns {number}
  */
function maxProbability (n, edges, succProb, start, end) {
  const maxProbs = new Array(n).fill(0)
  maxProbs[start] = 1

  for (let i = 0; i < n - 1; i += 1) {
    let breakable = true
    for (const [j, [u, v]] of edges.entries()) {
      const prob = succProb[j]
      const umax = maxProbs[u] * prob
      if (umax > maxProbs[v]) {
        maxProbs[v] = umax
        breakable = false
      }

      const vmax = maxProbs[v] * prob
      if (vmax > maxProbs[u]) {
        maxProbs[u] = vmax
        breakable = false
      }
    }

    if (breakable) {
      break
    }
  }

  return maxProbs[end]
}

function main () {
  const inputs = [
    { n: 3, edges: [[0, 1], [1, 2], [0, 2]], succProb: [0.5, 0.5, 0.2], start: 0, end: 2 },
    { n: 3, edges: [[0, 1], [1, 2], [0, 2]], succProb: [0.5, 0.5, 0.3], start: 0, end: 2 },
    { n: 3, edges: [[0, 1]], succProb: [0.5], start: 0, end: 2 }
  ]

  for (const { n, edges, succProb, start, end } of inputs) {
    const result = maxProbability(n, edges, succProb, start, end)
    console.log(result)
  }
}
main()

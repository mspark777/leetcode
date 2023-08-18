/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number} n
  * @param {number[][]} roads
  * @returns {number}
  */
function maximalNetworkRank (n, roads) {
  /** @type {Map<number, Set<number>>} */
  const adjacents = new Map()
  for (const [a, b] of roads) {
    const aSet = adjacents.get(a) ?? new Set()
    const bSet = adjacents.get(b) ?? new Set()

    aSet.add(b)
    bSet.add(a)
    adjacents.set(a, aSet)
    adjacents.set(b, bSet)
  }

  let result = 0
  for (let node0 = 0; node0 < n; node0 += 1) {
    const set0 = adjacents.get(node0) ?? new Set()
    const rank0 = set0.size ?? 0
    for (let node1 = node0 + 1; node1 < n; node1 += 1) {
      const rank1 = adjacents.get(node1)?.size ?? 0
      let rank = rank0 + rank1
      if (set0.has(node1)) {
        rank -= 1
      }

      result = Math.max(result, rank)
    }
  }

  return result
}

function main () {
  const inputs = [
    { n: 4, roads: [[0, 1], [0, 3], [1, 2], [1, 3]] },
    { n: 5, roads: [[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]] },
    { n: 8, roads: [[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]] }
  ]

  for (const { n, roads } of inputs) {
    const result = maximalNetworkRank(n, roads)
    console.log(result)
  }
}
main()

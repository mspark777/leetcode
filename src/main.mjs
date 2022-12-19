class UnionFind {
  /**
   * @param {number} x
   */
  constructor (n) {
    this.roots = new Array(n).fill(0).map((_, i) => i)
    this.ranks = new Array(n).fill(1)
  }

  /**
   * @param {number} x
   * @returns { number}
   */
  find (x) {
    const { roots } = this
    if (roots[x] !== x) {
      roots[x] = this.find(roots[x])
    }

    return roots[x]
  }

  /**
   * @param {number} x
   * @param {number} y
   * @returns {undefined}
   */
  union (x, y) {
    let rootx = this.find(x)
    let rooty = this.find(y)
    if (rootx !== rooty) {
      const { ranks } = this
      if (ranks[rootx] > ranks[rooty]) {
        const temp = rootx
        rootx = rooty
        rooty = temp
      }

      const { roots } = this
      roots[rootx] = rooty
      ranks[rooty] += ranks[rootx]
    }
  }
}

/**
 * @param {number} n
 * @param {number[][]} edges
 * @param {number} source
 * @param {number} destination
 * @returns {boolean}
 */
function validPath (n, edges, source, destination) {
  const uf = new UnionFind(n)
  for (const [x, y] of edges) {
    uf.union(x, y)
  }

  return uf.find(source) === uf.find(destination)
}

async function main () {
  const inputs = [
    {
      n: 3,
      edges: [[0, 1], [1, 2], [2, 0]],
      source: 0,
      destination: 2
    },
    {
      n: 6,
      edges: [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]],
      source: 0,
      destination: 5
    }
  ]

  for (const { n, edges, source, destination } of inputs) {
    const result = validPath(n, edges, source, destination)
    console.log(result)
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})

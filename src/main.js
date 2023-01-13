class LongestPath {
  constructor () {
    this.longestPath = 1
  }

  /**
   * @param {number} currentNode
   * @param {number[][]} children
   * @param {string} s
   * @returns {number}
   */
  dfs (currentNode, children, s) {
    let longestChain = 0
    let secondLongestChain = 0
    for (const child of children[currentNode]) {
      const longestChainStartingFromChild = this.dfs(child, children, s)
      if (s.charAt(currentNode) === s.charAt(child)) {
        continue
      }

      if (longestChainStartingFromChild > longestChain) {
        secondLongestChain = longestChain
        longestChain = longestChainStartingFromChild
      } else if (longestChainStartingFromChild > secondLongestChain) {
        secondLongestChain = longestChainStartingFromChild
      }
    }
    this.longestPath = Math.max(this.longestPath, longestChain + secondLongestChain + 1)
    return longestChain + 1
  }

  /**
   * @returns {number}
   */
  getResult () {
    return this.longestPath
  }
}

/**
 * @param {number[]} parent
 * @param {string} s
 * @returns {number}
 */
function longestPath (parent, s) {
  const children = Array.from(new Array(parent.length), () => [])
  for (let i = 1; i < parent.length; i += 1) {
    children[parent[i]].push(i)
  }

  const solution = new LongestPath()
  solution.dfs(0, children, s)
  return solution.getResult()
}

async function main () {
  const inputs = [
    { parent: [-1, 0, 0, 1, 1, 2], s: 'abacbe' },
    { parent: [-1, 0, 0, 0], s: 'aabc' }
  ]

  for (const { parent, s } of inputs) {
    const result = longestPath(parent, s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

class LongestPath {
  private longestPath: number
  public constructor () {
    this.longestPath = 1
  }

  public dfs (currentNode: number, children: number[][], s: string): number {
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

  public getResult (): number {
    return this.longestPath
  }
}

function longestPath (parent: number[], s: string): number {
  const children = Array.from(new Array<number[]>(parent.length), () => new Array<number>())
  for (let i = 1; i < parent.length; i += 1) {
    children[parent[i]].push(i)
  }

  const solution = new LongestPath()
  solution.dfs(0, children, s)
  return solution.getResult()
}

interface Input {
  readonly parent: number[]
  readonly s: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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

class UnionFind {
  private readonly parents: number[]
  private readonly ranks: number[]
  public constructor (size: number) {
    this.parents = new Array(size).fill(0).map((_, i) => i)
    this.ranks = new Array(size).fill(0)
  }

  public find (x: number): number {
    const { parents } = this
    if (parents[x] !== x) {
      parents[x] = this.find(parents[x])
    }

    return parents[x]
  }

  public unionSet (x: number, y: number): void {
    const { ranks, parents } = this
    const xset = this.find(x)
    const yset = this.find(y)
    if (xset === yset) {
      return
    }

    if (ranks[xset] < ranks[yset]) {
      parents[xset] = yset
    } else if (ranks[xset] > ranks[yset]) {
      parents[yset] = xset
    } else {
      parents[yset] = xset
      ranks[xset] += 1
    }
  }
}

function possibleBipartition (n: number, dislikes: number[][]): boolean {
  const adjusts = new Array<number[]>(n + 1).fill([]).map(() => new Array<number>())
  for (const [a, b] of dislikes) {
    adjusts[a].push(b)
    adjusts[b].push(a)
  }

  const uf = new UnionFind(n + 1)
  for (let i = 1; i <= n; i += 1) {
    for (const neighbor of adjusts[i]) {
      if (uf.find(i) === uf.find(neighbor)) {
        return false
      }

      uf.unionSet(adjusts[i][0], neighbor)
    }
  }

  return true
}

interface Input {
  readonly n: number
  readonly dislikes: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 4,
      dislikes: [[1, 2], [1, 3], [2, 4]]
    },
    {
      n: 3,
      dislikes: [[1, 2], [1, 3], [2, 3]]
    },
    {
      n: 5,
      dislikes: [[1, 2], [2, 3], [3, 4], [4, 5], [1, 5]]
    }
  ]

  for (const { n, dislikes } of inputs) {
    const result = possibleBipartition(n, dislikes)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

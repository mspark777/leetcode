import '@total-typescript/ts-reset'

class UnionFind {
  public readonly parents: number[]
  public readonly sizes: number[]
  public maxSize: number

  public constructor (n: number) {
    this.parents = new Array(n)
    this.sizes = new Array(n).fill(1)
    this.maxSize = 1

    for (let i = 0; i < n; i += 1) {
      this.parents[i] = i
    }
  }

  public find (x: number): number {
    if (x !== this.getParent(x)) {
      this.parents[x] = this.find(this.getParent(x))
    }

    return this.getParent(x)
  }

  public union (x: number, y: number): boolean {
    let rootX = this.find(x)
    let rootY = this.find(y)
    if (rootX !== rootY) {
      if (this.getSize(rootX) < this.getSize(rootY)) {
        const temp = rootX
        rootX = rootY
        rootY = temp
      }

      this.parents[rootY] = rootX
      this.sizes[rootX] += this.getSize(rootY)
      this.maxSize = Math.max(this.maxSize, this.getSize(rootX))
      return true
    }

    return false
  }

  private getParent (x: number): number {
    return this.parents[x] as number
  }

  private getSize (x: number): number {
    return this.sizes[x] as number
  }
}

function get2Dimension<T> (nums: T[][], r: number, c: number): T {
  return nums.at(r)?.at(c) as T
}

function get1Dimenstion<T> (nums: T[], i: number): T {
  return nums[i] as T
}

function findCriticalAndPseudoCriticalEdges (n: number, edges: number[][]): number[][] {
  const edgeCount = edges.length
  const newEdges = new Array<number[]>(edgeCount)
  for (let i = 0; i < edgeCount; i += 1) {
    const edge = get1Dimenstion(edges, i)
    const newEdge = new Array<number>(4)
    for (let j = 0; j < 3; j += 1) {
      newEdge[j] = get1Dimenstion(edge, j)
    }
    newEdge[3] = i
    newEdges[i] = newEdge
  }

  newEdges.sort((as, bs) => {
    const a = as[2] as number
    const b = bs[2] as number
    return a - b
  })

  const stdUF = new UnionFind(n)
  let stdWeight = 0
  for (const edge of newEdges) {
    const [a, b, w] = edge as [number, number, number]
    if (stdUF.union(a, b)) {
      stdWeight += w
    }
  }

  const result: number[][] = [[], []]
  for (let i = 0; i < edgeCount; i += 1) {
    const ignoreUF = new UnionFind(n)
    let ignoreWeight = 0
    for (let j = 0; j < edgeCount; j += 1) {
      if (i === j) {
        continue
      }

      if (ignoreUF.union(get2Dimension(newEdges, j, 0), get2Dimension(newEdges, j, 1))) {
        ignoreWeight += get2Dimension(newEdges, j, 2)
      }
    }

    if (ignoreUF.maxSize < n) {
      result.at(0)?.push(get2Dimension(newEdges, i, 3))
    } else if (ignoreWeight > stdWeight) {
      result.at(0)?.push(get2Dimension(newEdges, i, 3))
    } else {
      const forceUF = new UnionFind(n)
      forceUF.union(get2Dimension(newEdges, i, 0), get2Dimension(newEdges, i, 1))
      let forceWeight = get2Dimension(newEdges, i, 2)
      for (let j = 0; j < edgeCount; j += 1) {
        if (i === j) {
          continue
        }

        if (forceUF.union(get2Dimension(newEdges, j, 0), get2Dimension(newEdges, j, 1))) {
          forceWeight += get2Dimension(newEdges, j, 2)
        }
      }

      if (forceWeight === stdWeight) {
        result.at(1)?.push(get2Dimension(newEdges, i, 3))
      }
    }
  }

  return result
}

interface Input {
  readonly n: number
  readonly edges: number[][]
}

function main (): void {
  const inputs: Input[] = [
    { n: 5, edges: [[0, 1, 1], [1, 2, 1], [2, 3, 2], [0, 3, 2], [0, 4, 3], [3, 4, 3], [1, 4, 6]] },
    { n: 4, edges: [[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]] }
  ]

  for (const { n, edges } of inputs) {
    const result = findCriticalAndPseudoCriticalEdges(n, edges)
    console.log(result)
  }
}
main()

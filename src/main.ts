class Node {
  val: boolean
  isLeaf: boolean
  topLeft: Node | null
  topRight: Node | null
  bottomLeft: Node | null
  bottomRight: Node | null
  constructor (val?: boolean, isLeaf?: boolean, topLeft?: Node, topRight?: Node, bottomLeft?: Node, bottomRight?: Node) {
    this.val = (val === undefined ? false : val)
    this.isLeaf = (isLeaf === undefined ? false : isLeaf)
    this.topLeft = (topLeft === undefined ? null : topLeft)
    this.topRight = (topRight === undefined ? null : topRight)
    this.bottomLeft = (bottomLeft === undefined ? null : bottomLeft)
    this.bottomRight = (bottomRight === undefined ? null : bottomRight)
  }
}

function solve (grid: number[][], row: bigint, col: bigint, l: bigint): Node {
  if (l <= 1n) {
    return new Node(grid[Number(row)][Number(col)] === 1, true)
  }

  const next = l / 2n
  const topLeft = solve(grid, row, col, next)
  const topRight = solve(grid, row, col + next, next)
  const bottomLeft = solve(grid, row + next, col, next)
  const bottomRight = solve(grid, row + next, col + next, next)

  return (
    topLeft.isLeaf &&
    topRight.isLeaf &&
    bottomLeft.isLeaf &&
    bottomRight.isLeaf &&
    (topLeft.val === topRight.val) &&
    (topRight.val === bottomLeft.val) &&
    (bottomLeft.val === bottomRight.val)
  )
    ? new Node(topLeft.val, true)
    : new Node(false, false, topLeft, topRight, bottomLeft, bottomRight)
}

function construct (grid: number[][]): Node | null {
  return solve(grid, 0n, 0n, BigInt(grid.length))
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[0, 1], [1, 0]],
    [[1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0]]
  ]

  for (const grid of inputs) {
    const result = construct(grid)
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

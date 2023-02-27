class Node {
  /** @type {boolean} */
  val
  /** @type {boolean} */
  isLeaf
  /** @type {Node | null} */
  topLeft
  /** @type {Node | null} */
  topRight
  /** @type {Node | null} */
  bottomLeft
  /** @type {Node | null} */
  bottomRight

  /**
    * @param {boolean|undefined} val
    * @param {boolean|undefined} isLeaf
    * @param {Node|undefined} topLeft
    * @param {Node|undefined} topRight
    * @param {Node|undefined} bottomLeft
    * @param {Node|undefined} bottomRight
    */
  constructor (val, isLeaf, topLeft, topRight, bottomLeft, bottomRight) {
    this.val = (val === undefined ? false : val)
    this.isLeaf = (isLeaf === undefined ? false : isLeaf)
    this.topLeft = (topLeft === undefined ? null : topLeft)
    this.topRight = (topRight === undefined ? null : topRight)
    this.bottomLeft = (bottomLeft === undefined ? null : bottomLeft)
    this.bottomRight = (bottomRight === undefined ? null : bottomRight)
  }
}

/**
  * @param {number[][]} grid
  * @param {bigint} row
  * @param {bigint} col
  * @param {bigint} l
  * @returns {Node}
  */
function solve (grid, row, col, l) {
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

/**
  * @param {number[][]} grid
  * @returns {Node | null}
  */
function construct (grid) {
  return solve(grid, 0n, 0n, BigInt(grid.length))
}

async function main () {
  const inputs = [
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

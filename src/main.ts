function maxDistance (grid: number[][]): number {
  const LAND = 1
  const ROW_LEN = grid.length
  const COL_LEN = grid[0].length
  const MAX_DISTANCE = ROW_LEN + COL_LEN + 1

  const distances = Array.from(
    new Array(ROW_LEN),
    () => new Array<number>(COL_LEN).fill(MAX_DISTANCE)
  )

  for (let i = 0; i < ROW_LEN; i += 1) {
    for (let j = 0; j < COL_LEN; j += 1) {
      const cell = grid[i][j]
      if (cell === LAND) {
        distances[i][j] = 0
      } else {
        const distance = distances[i][j]
        distances[i][j] = Math.min(
          distance,
          Math.min(
            i > 0
              ? distances[i - 1][j] + 1
              : MAX_DISTANCE,
            j > 0
              ? distances[i][j - 1] + 1
              : MAX_DISTANCE
          )
        )
      }
    }
  }

  let result = Number.MIN_SAFE_INTEGER
  for (let i = ROW_LEN - 1; i >= 0; i--) {
    for (let j = COL_LEN - 1; j >= 0; j--) {
      let distance = distances[i][j]
      distance = Math.min(
        distance,
        Math.min(
          i < ROW_LEN - 1
            ? distances[i + 1][j] + 1
            : MAX_DISTANCE,
          j < ROW_LEN - 1
            ? distances[i][j + 1] + 1
            : MAX_DISTANCE
        )
      )

      distances[i][j] = distance
      result = Math.max(result, distance)
    }
  }

  return (result === 0) || (result === MAX_DISTANCE) ? -1 : result
}

async function main (): Promise<void> {
  const inputs = [
    [[1, 0, 1], [0, 0, 0], [1, 0, 1]],
    [[1, 0, 0], [0, 0, 0], [0, 0, 0]]
  ]

  for (const grid of inputs) {
    const result = maxDistance(grid)
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

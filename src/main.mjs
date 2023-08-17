/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[][]} mat
  * @returns {number[][]}
  */
function updateMatrix (mat) {
  const rowCount = mat.length
  const colCount = mat[0].length
  /** @type {number[][]} */
  const result = new Array(rowCount)
  for (let r = 0; r < rowCount; r += 1) {
    result[r] = new Array(colCount).fill(0)
  }

  /** @type {number[][]} */
  const queue = []
  const maxValue = rowCount * colCount

  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if (mat.at(r)?.at(c) === 0) {
        queue.push([r, c])
      } else {
        result[r][c] = maxValue
      }
    }
  }

  const directions = [[1, 0], [0, 1], [-1, 0], [0, -1]]
  for (let head = queue.shift(); head != null; head = queue.shift()) {
    const [row, col] = head
    const cell0 = result[row][col] + 1

    for (const [dr, dc] of directions) {
      const r = row + dr
      const c = col + dc
      if (r < 0) {
        continue
      } else if (r >= rowCount) {
        continue
      } else if (c < 0) {
        continue
      } else if (c >= colCount) {
        continue
      }

      const cell1 = result[r][c]
      if (cell1 <= cell0) {
        continue
      }

      queue.push([r, c])
      result[r][c] = cell0
    }
  }

  return result
}

function main () {
  const inputs = [
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 0, 0], [0, 1, 0], [1, 1, 1]]
  ]

  for (const mat of inputs) {
    const result = updateMatrix(mat)
    console.log(result)
  }
}
main()

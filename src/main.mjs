/**
 * @param {number[][]} mat
 * @return {number[][]}
 */
function diagonalSort (mat) {
  const rowCount = mat.length
  const colCount = mat[0].length

  const queues = new Map()
  for (let i = 0; i < rowCount; i += 1) {
    for (let j = 0; j < colCount; j += 1) {
      const key = i - j
      const queue = queues.get(key) ?? []
      queue.push(mat[i][j])
      queues.set(key, queue)
    }
  }

  for (const queue of queues.values()) {
    queue.sort((a, b) => b - a)
  }

  const result = new Array(rowCount)
  for (let i = 0; i < rowCount; i += 1) {
    const row = new Array(colCount)
    for (let j = 0; j < colCount; j += 1) {
      const key = i - j
      const queue = queues.get(key)
      row[j] = queue.pop()
    }

    result[i] = row
  }

  return result
}

async function main () {
  const inputs = [
    {
      mat: [[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]]
    },
    {
      mat: [[11, 25, 66, 1, 69, 7], [23, 55, 17, 45, 15, 52], [75, 31, 36, 44, 58, 8], [22, 27, 33, 25, 68, 4], [84, 28, 14, 11, 5, 50]]
    }
  ]

  for (const { mat } of inputs) {
    const result = diagonalSort(mat)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

function kthSmallest (matrix: number[][], k: number): number {
  const mlen = matrix.length
  let left = matrix[0][0]
  let right = matrix.at(-1)!.at(-1) as number
  while (left < right) {
    const mid = left + Math.trunc((right - left) / 2)
    let count = 0

    for (let i = 0; i < mlen; i += 1) {
      for (let j = mlen - 1; j >= 0; j -= 1) {
        if (matrix[i][j] <= mid) {
          count += j + 1
          break
        }
      }
    }

    if (count < k) {
      left = mid + 1
    } else {
      right = mid
    }
  }

  return left
}

interface Input {
  readonly matrix: number[][]
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      matrix: [[1, 5, 9], [10, 11, 13], [12, 13, 15]],
      k: 8
    },
    {
      matrix: [[-5]],
      k: 1
    },
    {
      matrix: [[-5, -4], [-5, -4]],
      k: 2
    },
    {
      matrix: [[1, 2], [1, 3]],
      k: 1
    }
  ]

  for (const input of inputs) {
    const { matrix, k } = input
    const result = kthSmallest(matrix, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

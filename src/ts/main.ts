function findKthPositive (arr: number[], k: number): number {
  let left = 0n
  let right = BigInt(arr.length)
  while (left < right) {
    const middle = (left + right) / 2n
    const pos = Number(middle)
    const n = arr[pos] - (pos + 1)
    if (n < k) {
      left = middle + 1n
    } else {
      right = middle
    }
  }

  return Number(left) + k
}

interface Input {
  readonly arr: number[]
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { arr: [2, 3, 4, 7, 11], k: 5 },
    { arr: [1, 2, 3, 4], k: 2 }
  ]

  for (const { arr, k } of inputs) {
    const result = findKthPositive(arr, k)
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

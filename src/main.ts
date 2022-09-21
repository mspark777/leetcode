function sumEvenAfterQueries (nums: number[], queries: number[][]): number[] {
  let sum = 0
  for (const num of nums) {
    if ((num % 2) === 0) {
      sum += num
    }
  }

  const result = new Array<number>(queries.length).fill(0)
  for (let i = 0; i < queries.length; i += 1) {
    const [val, index] = queries[i]
    let num = nums[index]
    if ((num % 2) === 0) {
      sum -= num
    }

    num += val
    if ((num % 2) === 0) {
      sum += num
    }

    nums[index] = num
    result[i] = sum
  }

  return result
}

interface Input {
  readonly nums: number[]
  readonly queries: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 2, 3, 4],
      queries: [[1, 0], [-3, 1], [-4, 0], [2, 3]]
    },
    {
      nums: [1],
      queries: [[4, 0]]
    }
  ]

  for (const { nums, queries } of inputs) {
    const result = sumEvenAfterQueries(nums, queries)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

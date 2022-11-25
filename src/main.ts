function sumSubarrayMins (arr: number[]): number {
  const stack = new Array<number>()
  const dp = new Array<number>(arr.length).fill(0)

  for (let i = 0; i < arr.length; i += 1) {
    const n = arr[i]
    while ((stack.length > 0) && (arr[stack.at(-1) as number] >= n)) {
      stack.pop()
    }

    if (stack.length > 0) {
      const top = stack.at(-1) as number
      dp[i] = dp[top] + (i - top) * n
    } else {
      dp[i] = (i + 1) * n
    }
    stack.push(i)
  }

  let result = 0
  for (const count of dp) {
    result += count
    result %= 1000000007
  }

  return result
}

async function main (): Promise<void> {
  const inputs: number[][] = [
    [3, 1, 2, 4],
    [11, 81, 94, 43, 3]
  ]

  for (const arr of inputs) {
    const result = sumSubarrayMins(arr)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

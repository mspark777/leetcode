function solve (n: number, memo: number[]): number {
  if (n === 0) {
    return 0
  } else if (n === 1) {
    return 1
  }

  if (memo[n] !== 0) {
    return memo[n]
  }

  if ((n % 2) === 0) {
    memo[n] = solve(Math.trunc(n / 2), memo)
  } else {
    memo[n] = solve(Math.trunc(n / 2), memo) + 1
  }

  return memo[n]
}

function countBits (n: number): number[] {
  const result = new Array<number>(n + 1).fill(0)

  for (let i = 1; i <= n; i += 1) {
    result[i] = solve(i, result)
  }

  return result
}

async function main (): Promise<void> {
  const inputs: number[] = [
    2, 5
  ]

  for (const n of inputs) {
    const result = countBits(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

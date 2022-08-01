function uniquePaths (m: number, n: number): number {
  let total = m + n - 2
  const r = Math.min(m, n) - 1

  let steps = 1

  for (let i = 1; i <= r; i += 1, total -= 1) {
    steps = Math.trunc(steps * total / i)
  }

  return steps
}

interface Input {
  readonly m: number
  readonly n: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      m: 3,
      n: 7
    },
    {
      m: 3,
      n: 2
    }
  ]

  for (const input of inputs) {
    const { m, n } = input
    const result = uniquePaths(m, n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

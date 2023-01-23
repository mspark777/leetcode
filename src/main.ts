function findJudge (n: number, trust: number[][]): number {
  const counts = new Array<number>(n).fill(0)
  for (const [from, to] of trust) {
    counts[from - 1] -= 1
    counts[to - 1] += 1
  }

  const JUDGE = n - 1
  for (const [person, count] of counts.entries()) {
    if (count === JUDGE) {
      return person + 1
    }
  }

  return -1
}

interface Input {
  readonly n: number
  readonly trust: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: 2, trust: [[1, 2]] },
    { n: 3, trust: [[1, 3], [2, 3]] },
    { n: 3, trust: [[1, 3], [2, 3], [3, 1]] },
    { n: 1, trust: [] }
  ]

  for (const { n, trust } of inputs) {
    const result = findJudge(n, trust)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

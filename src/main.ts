function poorPigs (buckets: number, minutesToDie: number, minutesToTest: number): number {
  return Math.ceil(Math.log(buckets) / Math.log(minutesToTest / minutesToDie + 1))
}

interface Input {
  buckets: number
  minutesToDie: number
  minutesToTest: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      buckets: 1000,
      minutesToDie: 15,
      minutesToTest: 60
    },
    {
      buckets: 4,
      minutesToDie: 15,
      minutesToTest: 15
    },
    {
      buckets: 4,
      minutesToDie: 15,
      minutesToTest: 30
    }
  ]

  for (const { buckets, minutesToDie, minutesToTest } of inputs) {
    const result = poorPigs(buckets, minutesToDie, minutesToTest)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

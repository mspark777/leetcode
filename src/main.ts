function countOdds (low: number, high: number): number {
  if ((low % 2) === 0) {
    low += 1
  }

  return low > high ? 0 : Math.trunc((high - low) / 2) + 1
}

async function main (): Promise<void> {
  const inputs = [
    [3, 7],
    [8, 10]
  ]

  for (const [low, high] of inputs) {
    const result = countOdds(low, high)
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

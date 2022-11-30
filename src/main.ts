function uniqueOccurrences (arr: number[]): boolean {
  const counts = new Map<number, number>()
  for (const n of arr) {
    const count = counts.get(n) ?? 0
    counts.set(n, count + 1)
  }

  const occurrences = new Set(counts.values())
  return occurrences.size === counts.size
}

async function main (): Promise<void> {
  const inputs: number[][] = [
    [1, 2, 2, 1, 1, 3],
    [1, 2],
    [-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]
  ]

  for (const arr of inputs) {
    const result = uniqueOccurrences(arr)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

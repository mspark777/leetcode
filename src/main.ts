import '@total-typescript/ts-reset'

function topKFrequent (nums: number[], k: number): number[] {
  const counts = new Map<number, number>()
  for (const num of nums) {
    const count = counts.get(num) ?? 0
    counts.set(num, count + 1)
  }

  return [...counts].sort((a, b) => b[1] - a[1]).slice(0, k).map(a => a[0])
}

interface Input {
  readonly nums: number[]
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { nums: [1, 1, 1, 2, 2, 3], k: 2 },
    { nums: [1], k: 1 }
  ]

  for (const { nums, k } of inputs) {
    const result = topKFrequent(nums, k)
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

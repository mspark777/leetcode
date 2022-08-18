function minSetSize (arr: number[]): number {
  const freqs = new Map<number, number>()
  for (const num of arr) {
    const freq = freqs.get(num) ?? 0
    freqs.set(num, freq + 1)
  }
  const pqueue: number[] = []
  for (const freq of freqs.values()) {
    pqueue.push(freq)
  }
  pqueue.sort((a, b) => b - a)

  let deleted = 0
  let result = 0
  const half = Math.trunc(arr.length / 2)
  for (const freq of pqueue) {
    deleted += freq
    result += 1
    if (deleted >= half) {
      return result
    }
  }

  return -1
}

interface Input {
  readonly arr: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      arr: [3, 3, 3, 3, 5, 5, 5, 2, 2, 7]
    },
    {
      arr: [7, 7, 7, 7, 7, 7]
    }
  ]

  for (const { arr } of inputs) {
    const result = minSetSize(arr)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

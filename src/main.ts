interface HeapNode {
  readonly word: string
  readonly count: number
}

function topKFrequent (words: string[], k: number): string[] {
  const counts = new Map<string, number>()
  for (const word of words) {
    const count = counts.get(word) ?? 0
    counts.set(word, count + 1)
  }

  const heap: HeapNode[] = []
  for (const [word, count] of counts.entries()) {
    heap.push({ word, count })
  }

  heap.sort((a, b) => {
    return a.count !== b.count
      ? b.count - a.count
      : a.word.localeCompare(b.word)
  })

  return heap.slice(0, k).map(n => n.word)
}

interface Input {
  readonly words: string[]
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      words: ['i', 'love', 'leetcode', 'i', 'love', 'coding'],
      k: 2
    },
    {
      words: ['the', 'day', 'is', 'sunny', 'the', 'the', 'the', 'sunny', 'is', 'is'],
      k: 4
    }
  ]

  for (const { words, k } of inputs) {
    const result = topKFrequent(words, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

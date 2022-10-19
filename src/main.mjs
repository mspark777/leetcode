/**
 * @param {string[]} words
 * @param {number} k
 * @returns {string[]}
*/
function topKFrequent (words, k) {
  const counts = new Map()
  for (const word of words) {
    const count = counts.get(word) ?? 0
    counts.set(word, count + 1)
  }

  const heap = []
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

async function main () {
  const inputs = [
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

function longestStrChain (words: string[]): number {
  words.sort((a, b) => a.length - b.length)
  let result = 0
  const dp = new Map<string, number>()

  for (const word of words) {
    let curLen = 1
    for (let i = 0; i < word.length; i += 1) {
      const predecessor = word.substring(0, i) + word.substring(i + 1)
      const preLen = dp.get(predecessor) ?? Number.MIN_SAFE_INTEGER
      curLen = Math.max(preLen + 1, curLen)
    }
    dp.set(word, curLen)
    result = Math.max(result, curLen)
  }

  return result
}

async function main (): Promise<void> {
  const inputs = [
    ['a', 'b', 'ba', 'bca', 'bda', 'bdca'],
    ['xbc', 'pcxbcf', 'xb', 'cxbc', 'pcxbc'],
    ['abcd', 'dbqca']
  ]

  for (const input of inputs) {
    const result = longestStrChain(input)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

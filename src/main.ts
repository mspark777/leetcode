function longestPalindrome (words: string[]): number {
  const counts = new Map<string, number>()
  for (const word of words) {
    const count = counts.get(word) ?? 0
    counts.set(word, count + 1)
  }

  let result = 0
  let central = false

  for (const [word, count] of counts) {
    const [first, second] = [...word]
    if (first === second) {
      if ((count % 2) === 0) {
        result += count
      } else {
        result += count - 1
        central = true
      }
    } else if (first.localeCompare(second) < 0) {
      const rword = `${second}${first}`
      if (counts.has(rword)) {
        result += 2 * Math.min(count, counts.get(rword) as number)
      }
    }
  }

  if (central) {
    result += 1
  }

  return result * 2
}

async function main (): Promise<void> {
  const inputs: string[][] = [
    ['lc', 'cl', 'gg'],
    ['ab', 'ty', 'yt', 'lc', 'cl', 'ab'],
    ['cc', 'll', 'xx']
  ]

  for (const words of inputs) {
    const result = longestPalindrome(words)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

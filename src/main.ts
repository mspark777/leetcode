function closeStrings (word1: string, word2: string): boolean {
  if (word1.length !== word2.length) {
    return false
  }

  const freq1 = new Map<string, number>()
  const freq2 = new Map<string, number>()
  for (let i = 0; i < word1.length; i += 1) {
    const ch1 = word1.charAt(i)
    const ch2 = word2.charAt(i)

    const cnt1 = freq1.get(ch1) ?? 0
    const cnt2 = freq2.get(ch2) ?? 0

    freq1.set(ch1, cnt1 + 1)
    freq2.set(ch2, cnt2 + 1)
  }

  if (freq1.size !== freq2.size) {
    return false
  } else if ([...freq1.keys()].some(k => !freq2.has(k))) {
    return false
  }

  const counts1 = [...freq1.values()].sort((a, b) => b - a)
  const counts2 = [...freq2.values()].sort((a, b) => b - a)
  for (let i = 0; i < counts1.length; i += 1) {
    if (counts1[i] !== counts2[i]) {
      return false
    }
  }

  return true
}

async function main (): Promise<void> {
  const inputs: string[][] = [
    ['abc', 'bca'],
    ['a', 'aa'],
    ['cabbba', 'abbccc']
  ]

  for (const [word1, word2] of inputs) {
    const result = closeStrings(word1, word2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

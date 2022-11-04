function reverseVowels (s: string): string {
  const vowels = new Set(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'])
  const words = [...s]
  let left = 0
  let right = words.length - 1

  while (left < right) {
    const l = words[left]
    const r = words[right]
    if (!vowels.has(l)) {
      left += 1
      continue
    } else if (!vowels.has(r)) {
      right -= 1
      continue
    }

    words[left] = r
    words[right] = l
    left += 1
    right -= 1
  }

  return words.join('')
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'hello',
    'leetcode'
  ]

  for (const s of inputs) {
    const result = reverseVowels(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

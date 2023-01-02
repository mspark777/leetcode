function detectCapitalUse(word: string): boolean {
  if (word === word.toUpperCase()) {
    return true
  } else if (word === word.toLowerCase()) {
    return true
  }


  return word === `${word[0].toUpperCase()}${word.substring(1).toLowerCase()}`
}

async function main(): Promise<void> {
  const inputs: string[] = [
    "USA",
    "Google",
    "leetcode",
    "FlaG"
  ]

  for (const word of inputs) {
    const result = detectCapitalUse(word)
    console.log(result)
  }
}

await main()

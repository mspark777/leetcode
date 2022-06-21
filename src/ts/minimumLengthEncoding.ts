export function minimumLengthEncoding (words: string[]): number {
  const filter = new Set<string>(words)
  for (const word of words) {
    for (let i = 1; i < word.length; i += 1) {
      filter.delete(word.substring(i))
    }
  }

  let result = 0
  for (const word of filter) {
    result += word.length + 1
  }

  return result
}

export function maxProduct (words: string[]): number {
  let result = 0
  const bits: number[] = new Array(words.length)
  const acode = 'a'.charCodeAt(0)
  for (let i = 0; i < words.length; i += 1) {
    const word = words[i]
    let curbit = 0
    for (const ch of word) {
      const j = ch.charCodeAt(0) - acode
      curbit |= 1 << j
    }
    bits[i] = curbit

    const wordLen = word.length
    for (let j = 0; j < i; j += 1) {
      const mask = curbit & bits[j]
      if (mask === 0) {
        const mul = wordLen * words[j].length
        result = Math.max(result, mul)
      }
    }
  }

  return result
}

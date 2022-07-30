const LETTER_COUNT = 26
const ACODE = 'a'.charCodeAt(0)

function getCounts (word: string): number[] {
  const counts = new Array<number>(LETTER_COUNT).fill(0)
  for (const ch of word) {
    const i = ch.charCodeAt(0) - ACODE
    counts[i] += 1
  }

  return counts
}

export function wordSubsets (words1: string[], words2: string[]): string[] {
  const counts2 = getCounts('')
  for (const word of words2) {
    const counts3 = getCounts(word)
    for (let i = 0; i < LETTER_COUNT; i += 1) {
      counts2[i] = Math.max(counts2[i], counts3[i])
    }
  }

  const result: string[] = []
  for (const word of words1) {
    const counts1 = getCounts(word)
    let ok = true
    for (let i = 0; i < LETTER_COUNT; i += 1) {
      if (counts1[i] < counts2[i]) {
        ok = false
        break
      }
    }

    if (ok) {
      result.push(word)
    }
  }

  return result
}

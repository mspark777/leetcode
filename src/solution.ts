function findPattern (word: string, pattern: string): boolean {
  const wmap = new Map<string, string>()
  const pmap = new Map<string, string>()

  for (let i = 0; i < word.length; i += 1) {
    const wc = word.charAt(i)
    const pc = pattern.charAt(i)
    if (pc.length < 1) {
      return false
    }

    if (!wmap.has(wc)) {
      wmap.set(wc, pc)
    }

    if (!pmap.has(pc)) {
      pmap.set(pc, wc)
    }

    if (wmap.get(wc) !== pc) {
      return false
    } else if (pmap.get(pc) !== wc) {
      return false
    }
  }

  return true
}

export function findAndReplacePattern (words: string[], pattern: string): string[] {
  const result: string[] = []
  for (const word of words) {
    if (findPattern(word, pattern)) {
      result.push(word)
    }
  }

  return result
}

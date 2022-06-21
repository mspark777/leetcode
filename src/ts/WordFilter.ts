export class WordFilter {
  private substrings: Map<string, number>
  constructor (words: string[]) {
    this.substrings = new Map<string, number>()
    for (let i = 0; i < words.length; i += 1) {
      const word = words[i]
      for (let j = 1; j <= word.length; j += 1) {
        const prefix = word.substring(0, j)
        for (let k = 0; k < word.length; k += 1) {
          const suffix = word.substring(k)
          const key = this.joinKey(prefix, suffix)
          this.substrings.set(key, i)
        }
      }
    }
  }

  f (prefix: string, suffix: string): number {
    const key = this.joinKey(prefix, suffix)
    return this.substrings.get(key) ?? -1
  }

  private joinKey (prefix: string, suffix: string): string {
    return [prefix, suffix].join('#')
  }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * var obj = new WordFilter(words)
 * var param_1 = obj.f(prefix,suffix)
 */

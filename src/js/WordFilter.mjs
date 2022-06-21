export class WordFilter {
  /**
   * @param {string[]} words
   */
  constructor (words) {
    this.substrings = new Map()
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

  /**
   * @param {string} prefix
   * @param {string} suffix
   * @return {number}
   */
  f (prefix, suffix) {
    const key = this.joinKey(prefix, suffix)
    return this.substrings.get(key) ?? -1
  }

  /**
   * @param {string} prefix
   * @param {string} suffix
   * @return {string}
   */
  joinKey (prefix, suffix) {
    return [prefix, suffix].join('#')
  }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * var obj = new WordFilter(words)
 * var param_1 = obj.f(prefix,suffix)
 */

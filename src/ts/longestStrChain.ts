namespace Version0 {
  export function dfs (word: string, wordSet: Set<string>, dp: Map<string, number>): number {
    if (dp.has(word)) {
      return dp.get(word)!
    }

    let result = 1
    for (let i = 0; i < word.length; i += 1) {
      const predecessor = word.substring(0, i) + word.substring(i + 1)
      if (wordSet.has(predecessor)) {
        result = Math.max(result, 1 + dfs(predecessor, wordSet, dp))
      }
    }
    dp.set(word, result)
    return result
  }
}

export function longestStrChain (words: string[]): number {
  let result = 0
  const wordSet = new Set(words)
  const dp = new Map<string, number>()

  for (const word of words) {
    result = Math.max(result, Version0.dfs(word, wordSet, dp))
  }

  return result
}

namespace Version1 {
  export function dfs (word: string, dp: Map<string, number>): number {
    const memo = dp.get(word) as number
    if (memo > 0) {
      return memo
    }

    let result = 1
    for (let i = 0; i < word.length; i += 1) {
      const predecessor = word.substring(0, i) + word.substring(i + 1)
      if (dp.has(predecessor)) {
        result = Math.max(result, 1 + dfs(predecessor, dp))
      }
    }
    dp.set(word, result)
    return result
  }
}

export function longestStrChain1 (words: string[]): number {
  let result = 0
  const dp = new Map<string, number>()
  for (const word of words) {
    dp.set(word, -1)
  }

  for (const word of words) {
    result = Math.max(result, Version1.dfs(word, dp))
  }

  return result
}

export function longestStrChain2 (words: string[]): number {
  words.sort((a, b) => a.length - b.length)
  let result = 0
  const dp = new Map<string, number>()

  for (const word of words) {
    let curLen = 1
    for (let i = 0; i < word.length; i += 1) {
      const predecessor = word.substring(0, i) + word.substring(i + 1)
      const preLen = dp.get(predecessor) ?? Number.MIN_SAFE_INTEGER
      curLen = Math.max(preLen + 1, curLen)
    }
    dp.set(word, curLen)
    result = Math.max(result, curLen)
  }

  return result
}

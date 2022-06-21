class UF {
  constructor (n) {
    this.arr = new Array(n).fill(0).map((_, i) => i)
  }

  root (i) {
    if (this.arr[i] === i) {
      return i
    }

    this.arr[i] = this.root(this.arr[i])
    return this.arr[i]
  }

  union (i, j) {
    i = this.root(i)
    j = this.root(j)
    if (i < j) {
      this.arr[j] = i
    } else {
      this.arr[i] = j
    }
  }
}
/**
 * @param {string} s
 * @param {number[][]} pairs
 * @return {string}
 */
export const smallestStringWithSwaps = function (s, pairs) {
  const uf = new UF(s.length)
  for (const pair of pairs) {
    uf.union(pair[0], pair[1])
  }

  const map = new Map()
  for (const i in s) {
    const root = uf.root(i)
    if (map.has(root)) {
      map.get(root).push(s[i])
    } else {
      map.set(root, [s[i]])
    }
  }

  map.forEach(v => v.sort((a, b) => b.localeCompare(a)))

  const result = []
  for (const i in s) {
    const root = uf.root(i)
    const chars = map.get(root)
    const char = chars.pop()
    result.push(char)
  }

  return result.join('')
}

export function hasAllCodes (s: string, k: number): boolean {
  const size = 1 << k
  let hash = 0
  let count = 0
  const mask = ~size
  const dp = new Array<boolean>(size)
  const zerocode = '0'.codePointAt(0)!
  for (let i = 0; i < s.length; i += 1) {
    hash <<= 1
    hash &= mask
    hash |= s.codePointAt(i)! - zerocode
    if ((i >= (k - 1)) && !dp[hash]) {
      dp[hash] = true
      count += 1
    }
  }

  return count === size
}

export function hasAllCodes1 (s: string, k: number): boolean {
  const subs = new Set<string>()
  for (let i = 0; i <= s.length - k; i += 1) {
    const sub = s.substring(i, k + i)
    subs.add(sub)
  }
  const count = 1 << k
  return subs.size === count
}

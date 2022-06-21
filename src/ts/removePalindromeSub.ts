export function removePalindromeSub (s: string): number {
  let i = 0
  let j = s.length - 1
  while (i < j) {
    const ci = s.charCodeAt(i)
    const cj = s.charCodeAt(j)
    if (ci !== cj) {
      return 2
    } else {
      i += 1
      j -= 1
    }
  }

  return 1
}

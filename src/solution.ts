export function isPalindrome (s: string): boolean {
  const test = s.replace(/[^0-9a-zA-Z]/g, '').toLowerCase()

  let i = 0
  let j = test.length - 1
  while (i < j) {
    if (test[i] !== test[j]) {
      return false
    }
    i += 1
    j -= 1
  }

  return true
}

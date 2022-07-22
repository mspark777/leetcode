export function isPalindrome (s) {
  const test = s.replace(/[^0-9a-zA-Z]/g, '').toLowerCase()

  let lo = 0
  let hi = test.length - 1
  while (lo <= hi) {
    if (test[lo] !== test[hi]) {
      return false
    }
    lo += 1
    hi -= 1
  }

  return true
}

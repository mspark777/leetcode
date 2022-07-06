export function fib (n) {
  if (n < 2) {
    return n
  }

  let prev0 = 0
  let prev1 = 1
  let cur = 1
  for (let i = 2; i <= n; i += 1) {
    cur = prev1 + prev0
    prev0 = prev1
    prev1 = cur
  }

  return cur
}

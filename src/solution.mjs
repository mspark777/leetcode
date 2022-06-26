export function maxScore (cardPoints, k) {
  const size = cardPoints.length - k
  let min = cardPoints.slice(0, size).reduce((acc, cur) => acc + cur, 0)
  let sum = min
  let cur = min

  for (let i = 0; i < k; i += 1) {
    const p = cardPoints[i + size]
    sum += p
    cur += p - cardPoints[i]
    min = Math.min(min, cur)
  }

  return sum - min
}

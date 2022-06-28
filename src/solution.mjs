export function minDeletions (s) {
  const frequency = new Array(26).fill(0)
  const acode = 'a'.charCodeAt(0)
  for (let i = 0; i < s.length; i += 1) {
    const code = s.charCodeAt(i)
    frequency[code - acode] += 1
  }

  frequency.sort((a, b) => b - a)

  let result = 0
  let max = s.length
  for (let i = 0; i < frequency.length; i += 1) {
    const f = frequency[i]
    if (f < 1) {
      break
    }

    if (f > max) {
      result += f - max
      frequency[i] = max
    }
    max = Math.max(0, frequency[i] - 1)
  }

  return result
}

export function minPartitions (n) {
  let max = 0
  for (const ch of n) {
    max = Math.max(max, Number(ch))
  }

  return max
}

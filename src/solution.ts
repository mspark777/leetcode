export function minPartitions (n: string): number {
  let max = 0
  for (const ch of n) {
    max = Math.max(max, Number(ch))
  }

  return max
}

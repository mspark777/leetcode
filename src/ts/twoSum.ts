export function twoSum (numbers: number[], target: number): number[] {
  let i = 0
  let j = numbers.length - 1
  while (i < j) {
    const ni = numbers[i]
    const nj = numbers[j]
    const ns = ni + nj
    if (ns < target) {
      i += 1
    } else if (ns > target) {
      j -= 1
    } else {
      break
    }
  }

  return [i + 1, j + 1]
}

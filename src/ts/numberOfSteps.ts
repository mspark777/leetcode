export function numberOfSteps (num: number): number {
  let result = 0
  while (num !== 0) {
    result += ((num & 1) === 1) ? 2 : 1
    num >>>= 1
  }

  return Math.max(result - 1, 0)
}

export function numberOfSteps1 (num: number): number {
  let result = -1
  while (num !== 0) {
    result += ((num & 1) === 1) ? 2 : 1
    num >>>= 1
  }

  return Math.max(result, 0)
}

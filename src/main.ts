import '@total-typescript/ts-reset'

function peakIndexInMountainArray (arr: number[]): number {
  let left = 0
  let right = arr.length - 1
  while (left < right) {
    const mid = Math.trunc((left + right) / 2)
    const l = arr.at(mid) as number
    const r = arr.at(mid + 1) as number
    if (l < r) {
      left = mid + 1
    } else {
      right = mid
    }
  }

  return left
}

function main (): void {
  const inputs: number[][] = [
    [0, 1, 0],
    [0, 2, 1, 0],
    [0, 10, 5, 2]
  ]

  for (const arr of inputs) {
    const result = peakIndexInMountainArray(arr)
    console.log(result)
  }
}
main()

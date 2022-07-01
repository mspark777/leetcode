export function maximumUnits (boxTypes: number[][], truckSize: number): number {
  boxTypes.sort((a, b) => b[1] - a[1])

  let result = 0
  for (const boxType of boxTypes) {
    const count = boxType[0]
    const units = boxType[1]
    const min = Math.min(truckSize, count)
    result += units * min
    truckSize -= min
    if (truckSize <= 0) {
      break
    }
  }

  return result
}

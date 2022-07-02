export function maxArea (h, w, horizontalCuts, verticalCuts) {
  horizontalCuts.sort((a, b) => a - b)
  verticalCuts.sort((a, b) => a - b)

  let maxH = Math.max(horizontalCuts[0], h - horizontalCuts.at(-1))
  for (let i = 1; i < horizontalCuts.length; i += 1) {
    maxH = Math.max(maxH, horizontalCuts[i] - horizontalCuts[i - 1])
  }

  let maxW = Math.max(verticalCuts[0], w - verticalCuts.at(-1))
  for (let i = 1; i < verticalCuts.length; i += 1) {
    maxW = Math.max(maxW, verticalCuts[i] - verticalCuts[i - 1])
  }

  const result = (BigInt(maxH) * BigInt(maxW)) % 1000000007n
  return Number(result)
}

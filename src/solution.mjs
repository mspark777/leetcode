function findMaxDiffIndex (arr) {
  let max = arr[0]
  let result = 0
  for (let i = 1; i < arr.length; i += 1) {
    const val = arr[i]
    if (max < val) {
      max = val
      result = i
    }
  }

  return result
}

export function furthestBuilding (heights, bricks, ladders) {
  const maxHeap = []

  for (let i = 1; i < heights.length; i += 1) {
    const diff = heights[i] - heights[i - 1]
    if (diff <= 0) {
      continue
    }

    maxHeap.push(diff)
    bricks -= diff
    if (bricks < 0) {
      const maxIndex = findMaxDiffIndex(maxHeap)
      const max = maxHeap[maxIndex]
      maxHeap.splice(maxIndex, 1)
      bricks += max
      ladders -= 1
    }

    if (ladders < 0) {
      return i - 1
    }
  }

  return heights.length - 1
}

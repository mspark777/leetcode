function findMaxDiffIndex (arr: number[]):number {
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

function furthestBuilding (heights: number[], bricks: number, ladders: number): number {
  const maxHeap: number[] = []

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

async function main (): Promise<void> {
  const inputs = [
    { heights: [4, 2, 7, 6, 9, 14, 12], bricks: 5, leadders: 1 },
    { heights: [4, 12, 2, 7, 3, 18, 20, 3, 19], bricks: 10, leadders: 2 },
    { heights: [14, 3, 19, 3], bricks: 17, leadders: 0 },
    { heights: [1, 5, 1, 2, 3, 4, 10000], bricks: 4, leadders: 1 }
  ]

  for (const input of inputs) {
    const result = furthestBuilding(input.heights, input.bricks, input.leadders)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

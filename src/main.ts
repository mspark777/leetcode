function trap (height: number[]): number {
  let left = 0
  let right = height.length - 1
  let leftMax = 0
  let rightMax = 0
  let result = 0

  while (left < right) {
    const lheight = height[left]
    const rheight = height[right]
    if (lheight < rheight) {
      left += 1
      if (lheight >= leftMax) {
        leftMax = lheight
      } else {
        result += leftMax - lheight
      }
    } else {
      right -= 1
      if (rheight >= rightMax) {
        rightMax = rheight
      } else {
        result += rightMax - rheight
      }
    }
  }

  return result
}

interface Input {
  readonly height: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      height: [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
    },
    {
      height: [4, 2, 0, 3, 2, 5]
    }
  ]

  for (const { height } of inputs) {
    const result = trap(height)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

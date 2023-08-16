import '@total-typescript/ts-reset'

function maxSlidingWindow (nums: number[], k: number): number[] {
  const queue: number[] = []
  const result: number[] = []

  for (let i = 0; i < k; i += 1) {
    while (queue.length > 0) {
      const taili = queue.at(-1) as number
      const tail = nums[taili] as number
      const num = nums[i] as number
      if (num >= tail) {
        queue.pop()
      } else {
        break
      }
    }

    queue.push(i)
  }

  result.push(nums[queue[0] as number] as number)

  for (let i = k; i < nums.length; i += 1) {
    const head = queue[0] as number
    if (head === (i - k)) {
      queue.shift()
    }

    while (queue.length > 0) {
      const taili = queue.at(-1) as number
      const tail = nums[taili] as number
      const num = nums[i] as number
      if (num >= tail) {
        queue.pop()
      } else {
        break
      }
    }

    queue.push(i)
    result.push(nums[queue[0] as number] as number)
  }

  return result
}

interface Input {
  readonly nums: number[]
  readonly k: number
}

function main (): void {
  const inputs: Input[] = [
    { nums: [1, 3, -1, -3, 5, 3, 6, 7], k: 3 },
    { nums: [1], k: 1 }
  ]

  for (const { nums, k } of inputs) {
    const result = maxSlidingWindow(nums, k)
    console.log(result)
  }
}
main()

function maximum69Number (num: number): number {
  const nums = [...num.toString()]
  for (let i = 0; i < nums.length; i += 1) {
    if (nums[i] === '6') {
      nums[i] = '9'
      break
    }
  }

  return Number(nums.join(''))
}

async function main (): Promise<void> {
  const inputs: number[] = [
    9669, 9996, 9999
  ]

  for (const num of inputs) {
    const result = maximum69Number(num)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

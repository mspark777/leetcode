import '@total-typescript/ts-reset'

function div (left: number, right: number): number {
  return Number(BigInt(left) / BigInt(right))
}

function mod (left: number, right: number): number {
  return Number(BigInt(left) % BigInt(right))
}

function getPermutation (n: number, k: number): string {
  if ([0, 1].includes(n)) {
    return n.toString()
  }

  const nums = new Array<number>(n)
  let fact = 1
  for (let i = 1; i < n; i += 1) {
    nums[i - 1] = i
    fact *= i
  }
  nums[n - 1] = n

  k -= 1
  const result: number[] = []
  while (nums.length > 0) {
    const pos = div(k, fact)
    result.push(nums[pos] as number)
    nums.splice(pos, 1)
    if (nums.length < 1) {
      break
    }

    k = mod(k, fact)
    fact = div(fact, nums.length)
  }

  return result.join('')
}

function main (): void {
  const inputs: Array<[number, number]> = [
    [3, 3],
    [4, 9],
    [3, 1]
  ]

  for (const [n, k] of inputs) {
    const result = getPermutation(n, k)
    console.log(result)
  }
}
main()

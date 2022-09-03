/**
 * @param {number} n
 * @param {number} k
 * @return {number[]}
 */
function numsSameConsecDiff (n, k) {
  const stack = []
  const result = []
  for (let i = 1; i <= 9; i += 1) {
    stack.push({
      len: n - 1,
      num: i,
      digit: i
    })
  }

  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { len, num, digit } = top
    if (len === 0) {
      result.push(num)
      continue
    }

    for (let i = 0; i < 10; i += 1) {
      if (Math.abs(i - digit) === k) {
        stack.push({
          len: len - 1,
          num: num * 10 + i,
          digit: i
        })
      }
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      n: 3,
      k: 7
    },
    {
      n: 2,
      k: 1
    }
  ]

  for (const { n, k } of inputs) {
    const result = numsSameConsecDiff(n, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

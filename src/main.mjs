class MyMap {
  constructor () {
    this.m = new Map()
  }

  get (k) {
    return this.m.get(k) ?? 0
  }

  add (k, n) {
    const v = this.get(k)
    this.m.set(k, v + n)
  }
}

/**
 * @param {number[]} nums
 * @return {boolean}
 */
function isPossible (nums) {
  const lefts = new MyMap()
  const ends = new MyMap()

  for (const num of nums) {
    lefts.add(num, 1)
  }

  for (const cur of nums) {
    if (lefts.get(cur) === 0) {
      continue
    }

    lefts.add(cur, -1)

    const before1 = cur - 1
    const ebefore1 = ends.get(before1)
    if (ebefore1 > 0) {
      ends.add(before1, -1)
      ends.add(cur, 1)
      continue
    }

    const after1 = cur + 1
    const after2 = cur + 2
    const lafter1 = lefts.get(after1)
    const lafter2 = lefts.get(after2)
    if ((lafter1 > 0) && (lafter2 > 0)) {
      lefts.add(after1, -1)
      lefts.add(after2, -1)
      ends.add(after2, 1)
      continue
    }

    return false
  }

  return true
}

async function main () {
  const inputs = [
    {
      nums: [1, 2, 3, 3, 4, 5]
    },
    {
      nums: [1, 2, 3, 3, 4, 4, 5, 5]
    },
    {
      nums: [1, 2, 3, 4, 4, 5]
    }
  ]

  for (const { nums } of inputs) {
    const result = isPossible(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})

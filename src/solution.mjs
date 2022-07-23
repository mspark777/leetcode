function merge (pairs, l, mid, r, result) {
  let i = l
  let j = mid + 1
  let k = 0
  const temp = new Array(r - l + 1)
  let count = 0

  while (i <= mid && j <= r) {
    const ip = pairs[i]
    const jp = pairs[j]
    if (ip.n <= jp.n) {
      result[ip.i] += count
      temp[k] = ip

      i += 1
      k += 1
    } else {
      count += 1
      temp[k] = jp
      k += 1
      j += 1
    }
  }

  while (i <= mid) {
    const p = pairs[i]
    result[p.i] += count
    temp[k] = p
    i += 1
    k += 1
  }

  while (j <= r) {
    temp[k] = pairs[j]
    k += 1
    j += 1
  }

  for (i = 0; i < temp.length; i += 1) {
    pairs[l + i] = temp[i]
  }
}

function mergesort (pairs, l, r, result) {
  if (l >= r) {
    return
  }

  const mid = Math.trunc((l + r) / 2)
  mergesort(pairs, l, mid, result)
  mergesort(pairs, mid + 1, r, result)
  merge(pairs, l, mid, r, result)
}

export function countSmaller (nums) {
  const n = nums.length
  const result = new Array(n).fill(0)
  const pairs = new Array(n)
  for (let i = n - 1; i >= 0; i -= 1) {
    pairs[i] = { n: nums[i], i }
  }

  mergesort(pairs, 0, n - 1, result)
  return result
}

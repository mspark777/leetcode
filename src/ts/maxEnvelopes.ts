function search (arr: number[], begin: number, end: number, value: number): number {
  while (begin < end) {
    const mindex = Math.trunc((begin + end) / 2)
    const mvalue = arr[mindex]

    if (mvalue < value) {
      begin = mindex + 1
    } else if (mvalue > value) {
      end = mindex
    } else {
      return mindex
    }
  }

  return -(begin + 1)
}

export function maxEnvelopes (envelopes: number[][]): number {
  envelopes.sort((a, b) => {
    const diff = a[0] - b[0]
    return diff !== 0 ? diff : b[1] - a[1]
  })

  let result = 0
  const dp = new Array(envelopes.length).fill(0)
  for (const envelope of envelopes) {
    const height = envelope[1]
    let index = search(dp, 0, result, height)
    if (index < 0) {
      index = -(index + 1)
    }
    dp[index] = height
    if (index === result) {
      result += 1
    }
  }

  return result
}

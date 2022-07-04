function count (n) {
  const c = (n * (n + 1)) / 2
  return Math.trunc(c)
}

export function candy (ratings) {
  if (ratings.length < 2) {
    return ratings.length
  }

  let candies = 0
  let up = 0
  let down = 0
  let oldSlope = 0

  for (let i = 1; i < ratings.length; i += 1) {
    const cur = ratings[i]
    const prev = ratings[i - 1]
    let slope = 0
    if (cur > prev) {
      slope = 1
    } else if (cur < prev) {
      slope = -1
    }

    if (((oldSlope > 0) && (slope === 0)) ||
      ((oldSlope < 0) && (slope >= 0))) {
      candies += count(up) + count(down) + Math.max(up, down)
      up = 0
      down = 0
    }

    if (slope > 0) {
      up += 1
    } else if (slope < 0) {
      down += 1
    } else {
      candies += 1
    }

    oldSlope = slope
  }

  candies += count(up) + count(down) + Math.max(up, down) + 1
  return candies
}

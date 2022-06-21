/**
 * @param {number[][]} points
 * @return {number}
 */
export const minCostConnectPoints = function (points) {
  if (points.length < 2) {
    return 0
  } else if (points.length === 2) {
    return Math.abs(points[0][0] - points[1][0]) + Math.abs(points[0][1] - points[1][1])
  }

  const distances = []
  for (let i = 0; i < points.length; i += 1) {
    const ip = points[i]
    for (let j = i + 1; j < points.length; j += 1) {
      const jp = points[j]
      const distance = Math.abs(ip[0] - jp[0]) + Math.abs(ip[1] - jp[1])
      distances.push({ i, j, distance })
    }
  }

  distances.sort((a, b) => b.distance - a.distance)

  const memos = points.map(() => [])
  const maxLineCount = points.length - 1
  let lineCount = 0
  let cost = 0

  const needConnect = (i, j, paths) => {
    paths.add(i)
    if (memos[i].includes(j)) {
      return false
    }

    for (const k of memos[i]) {
      if (paths.has(k)) {
        continue
      } else if (k === j) {
        continue
      }

      if (!needConnect(k, j, paths)) {
        return false
      }
    }

    return true
  }

  while (lineCount < maxLineCount) {
    const distance = distances.pop()
    const i = distance.i
    const j = distance.j
    if (needConnect(i, j, new Set())) {
      lineCount += 1
      cost += distance.distance
    }
    memos[i].push(j)
    memos[j].push(i)
  }

  return cost
}

export function findPaths (m, n, maxMove, startRow, startColumn) {
  const MODULO = 1000000007
  const createDP = () => {
    const dp = new Array(m)
    for (let i = 0; i < m; i += 1) {
      dp[i] = new Array(n).fill(0)
    }
    return dp
  }
  let dp = createDP()
  dp[startRow][startColumn] = 1
  let count = 0
  for (let moves = 1; moves <= maxMove; moves += 1) {
    const temp = createDP()
    for (let i = 0; i < m; i += 1) {
      for (let j = 0; j < n; j += 1) {
        if (i === (m - 1)) {
          count = (count + dp[i][j]) % MODULO
        }

        if (j === (n - 1)) {
          count = (count + dp[i][j]) % MODULO
        }

        if (i === 0) {
          count = (count + dp[i][j]) % MODULO
        }

        if (j === 0) {
          count = (count + dp[i][j]) % MODULO
        }

        let ti = 0
        if (i > 0) {
          ti += dp[i - 1][j]
        }

        if (i < (m - 1)) {
          ti += dp[i + 1][j]
        }
        ti %= MODULO

        let tj = 0
        if (j > 0) {
          tj += dp[i][j - 1]
        }

        if (j < (n - 1)) {
          tj += dp[i][j + 1]
        }
        tj %= MODULO

        temp[i][j] = (ti + tj) % MODULO
      }
    }

    dp = temp
  }

  return count
}

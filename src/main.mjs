/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number} amount
  * @param {number[]} coins
  * @returns {number}
  */
function change (amount, coins) {
  /** @type {number[]} */
  const dp = new Array(amount + 1).fill(0)
  dp[0] = 1

  for (let i = coins.length - 1; i >= 0; i -= 1) {
    const coin = coins[i]
    for (let j = coin; j <= amount; j += 1) {
      dp[j] += dp[j - coin]
    }
  }

  return dp[amount]
}

function main () {
  const inputs = [
    { amount: 5, coins: [1, 2, 5] },
    { amount: 3, coins: [2] },
    { amount: 10, coins: [10] }
  ]

  for (const { amount, coins } of inputs) {
    const result = change(amount, coins)
    console.log(result)
  }
}
main()

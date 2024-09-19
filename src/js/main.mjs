/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} ch
 * @return {boolean}
 */
function isDigit(ch) {
  const ZERO = "0".charCodeAt(0);
  const NINE = "9".charCodeAt(0);
  const code = ch.charCodeAt(0);
  return ZERO <= code && code <= NINE;
}

/**
 * @param {string} expression
 * @return {number[][][]}
 */
function initializeBaseCases(expression) {
  const dp = [];

  for (let i = 0; i < expression.length; i += 1) {
    const row = [];
    for (let j = 0; j < expression.length; j += 1) {
      row.push([]);
    }

    dp.push(row);
  }

  for (let i = 0; i < expression.length; i += 1) {
    for (let j = 0; j < expression.length; j += 1) {
      dp[i][j] = [];
    }
  }

  const ZERO = "0".charCodeAt(0);
  for (let i = 0; i < expression.length; i += 1) {
    if (!isDigit(expression.at(i))) {
      continue;
    }

    const dig1 = expression.charCodeAt(i) - ZERO;
    if (i + 1 < expression.length && isDigit(expression.at(i + 1))) {
      const dig2 = expression.charCodeAt(i + 1) - ZERO;
      const n = dig1 * 10 + dig2;
      dp[i][i + 1].push(n);
    }
    dp[i][i].push(dig1);
  }

  return dp;
}

/**
 * @param {string} op
 * @param {number[]} leftResults
 * @param {number[]} rightResults
 * @param {number[]} results
 * @returns {undefined}
 */
function computeResults(op, leftResults, rightResults, results) {
  for (const leftValue of leftResults) {
    for (const rightValue of rightResults) {
      if (op === "+") {
        results.push(leftValue + rightValue);
      } else if (op === "-") {
        results.push(leftValue - rightValue);
      } else if (op === "*") {
        results.push(leftValue * rightValue);
      }
    }
  }
}

/**
 * @param {string} expression
 * @param {number[][][]} dp
 * @param {number} start
 * @param {number} end
 * @returns {undefined}
 */
function processSubexpression(expression, dp, start, end) {
  for (let split = start; split <= end; split += 1) {
    if (isDigit(expression.at(split))) {
      continue;
    }

    const leftResults = dp[start][split - 1];
    const rightResults = dp[split + 1][end];

    computeResults(
      expression.at(split),
      leftResults,
      rightResults,
      dp[start][end],
    );
  }
}

/**
 * @param {string} expression
 * @return {number[]}
 */
function diffWaysToCompute(expression) {
  const dp = initializeBaseCases(expression);

  for (let i = 3; i <= expression.length; i += 1) {
    for (let j = 0; j + i - 1 < expression.length; j += 1) {
      const k = i + j - 1;
      processSubexpression(expression, dp, j, k);
    }
  }

  return dp[0][expression.length - 1];
}

function main() {
  const inputs = ["2-1-1", "2*3-4*5"];

  for (const expression of inputs) {
    const result = diffWaysToCompute(expression);
    console.log(result);
  }
}
main();

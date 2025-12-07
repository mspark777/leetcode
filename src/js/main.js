class Calculator {
  /**
   * @param {number} value
   */
  constructor(value) {
    this.value = value;
  }

  /**
   * @param {number} value
   * @return {Calculator}
   */
  add(value) {
    this.value += value;
    return this;
  }

  /**
   * @param {number} value
   * @return {Calculator}
   */
  subtract(value) {
    this.value -= value;
    return this;
  }

  /**
   * @param {number} value
   * @return {Calculator}
   */
  multiply(value) {
    this.value *= value;
    return this;
  }

  /**
   * @param {number} value
   * @return {Calculator}
   */
  divide(value) {
    if (value === 0) {
      throw new Error("Division by zero is not allowed");
    }

    this.value /= value;
    return this;
  }

  /**
   * @param {number} value
   * @return {Calculator}
   */
  power(value) {
    this.value = Math.pow(this.value, value);
    return this;
  }

  /**
   * @return {number}
   */
  getResult() {
    return this.value;
  }
}

/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var minimumDifference = function (nums, k) {
  if (nums.length < k) {
    return 0;
  }

  nums.sort((a, b) => a - b);
  let result = Number.MAX_SAFE_INTEGER;

  for (let i = 0; i < nums.length - k + 1; i += 1) {
    result = Math.min(result, nums[i + k - 1] - nums[i]);
  }

  return result;
};

/**
 * @typedef Input
 * @property {number[]} nums
 * @property {number} k
 */

/**
 * @return {undefined}
 */
function main() {
  /** @type Input[] */
  const inputs = [
    {
      nums: [90],
      k: 1,
    },
    {
      nums: [9, 4, 1, 7],
      k: 2,
    },
  ];

  for (const input of inputs) {
    const result = minimumDifference(input.nums, input.k);
    console.log(result);
  }
}
main();

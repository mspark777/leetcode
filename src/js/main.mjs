/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} nums
 * @return {string}
 */
function largestNumber(nums) {
  const strs = nums
    .map((n) => n.toString())
    .sort((left, right) => {
      const l = left + right;
      const r = right + left;
      return r.localeCompare(l);
    });

  if (strs.at(0) === "0") {
    return "0";
  }

  return strs.join("");
}

function main() {
  const inputs = [
    [10, 2],
    [3, 30, 34, 5, 9],
  ];

  for (const nums of inputs) {
    const result = largestNumber(nums);
    console.log(result);
  }
}
main();

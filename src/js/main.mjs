/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} g
 * @param {number[]} s
 * @return {number}
 */
function findContentChildren(g, s) {
  g.sort((l, r) => l - r);
  s.sort((l, r) => l - r);

  let result = 0;
  for (const content of s) {
    if (content >= g[result]) {
      result += 1;
    }

    if (result >= g.length) {
      break;
    }
  }

  return result;
}

function main() {
  const inputs = [
    [
      [1, 2, 3],
      [1, 1],
    ],
    [
      [1, 2],
      [1, 2, 3],
    ],
  ];

  for (const [g, s] of inputs) {
    const result = findContentChildren(g, s);
    console.log(result);
  }
}
main();

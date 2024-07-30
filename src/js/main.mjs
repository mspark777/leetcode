/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string[]} list1
 * @param {string[]} list2
 * @return {string[]}
 */
function findRestaurant(list1, list2) {
  const map1 = new Map();
  for (const [i, name] of list1.entries()) {
    map1.set(name, i);
  }

  let result = [];
  let min_sum = Number.MAX_SAFE_INTEGER;
  for (const [i, name2] of list2.entries()) {
    if (i > min_sum) {
      break;
    }

    const at1 = map1.get(name2);
    if (at1 == null) {
      continue;
    }

    const sum = i + at1;
    if (sum < min_sum) {
      result = [name2];
      min_sum = sum;
    } else if (sum == min_sum) {
      result.push(name2);
    }
  }

  return result;
}

function main() {
  const inputs = [
    [
      ["Shogun", "Tapioca Express", "Burger King", "KFC"],
      [
        "Piatti",
        "The Grill at Torrey Pines",
        "Hungry Hunter Steakhouse",
        "Shogun",
      ],
    ],
    [
      ["Shogun", "Tapioca Express", "Burger King", "KFC"],
      ["KFC", "Shogun", "Burger King"],
    ],
    [
      ["happy", "sad", "good"],
      ["sad", "happy", "good"],
    ],
  ];

  for (const [list1, list2] of inputs) {
    const result = findRestaurant(list1, list2);
    console.log(result);
  }
}
main();

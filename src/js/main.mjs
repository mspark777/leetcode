/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number} l
 * @param {number} r
 * @returns {number}
 */
function div(l, r) {
  return Number(BigInt(l) / BigInt(r));
}

/**
 * @param {number} l
 * @param {number} r
 * @returns {number}
 */
function mod(l, r) {
  return Number(BigInt(l) % BigInt(r));
}

/**
 * @param {number[]} skills
 * @return {number}
 */
function dividePlayers(skills) {
  const n = skills.length;
  let totalSkill = 0;
  /** @type {Map<number, number>} */
  const skillMap = new Map();

  for (const s of skills) {
    totalSkill += s;

    const count = skillMap.get(s) ?? 0;
    skillMap.set(s, count + 1);
  }

  if (mod(totalSkill, div(n, 2)) !== 0) {
    return -1;
  }

  let targetSkill = div(totalSkill, div(n, 2));
  let result = 0;

  for (const [skill, count] of skillMap) {
    const partnerSkill = targetSkill - skill;
    const partnerCount = skillMap.get(partnerSkill);
    if (partnerCount == null || partnerCount !== count) {
      return -1;
    }

    result += skill * partnerSkill * count;
  }

  return div(result, 2);
}

function main() {
  const inputs = [
    [3, 2, 5, 1, 3, 4],
    [3, 4],
    [1, 1, 2, 3],
  ];

  for (const skill of inputs) {
    const result = dividePlayers(skill);
    console.log(result);
  }
}
main();

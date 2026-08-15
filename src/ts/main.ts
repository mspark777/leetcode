import "@total-typescript/ts-reset";

function numFriendRequests(ages: number[]): number {
  const counts = new Map<number, number>();
  for (const age of ages) {
    const count = counts.get(age) ?? 0;
    counts.set(age, count + 1);
  }

  let result = 0;
  for (let ageA = 0; ageA <= 120; ageA += 1) {
    const countA = counts.get(ageA);
    if (countA == null) {
      continue;
    }

    for (let ageB = 0; ageB <= 120; ageB += 1) {
      const countB = counts.get(ageB);
      if (countB == null) {
        continue;
      }

      if (ageA * 0.5 + 7 >= ageB) {
        continue;
      }
      if (ageA < ageB) {
        continue;
      }
      if (ageA < 100 && 100 < ageB) {
        continue;
      }

      result += countA * countB;
      if (ageA == ageB) {
        result -= countA;
      }
    }
  }

  return result;
}

interface Input {
  ages: number[];
}

function main(): void {
  const inputs: Input[] = [{ ages: [16, 16] }, { ages: [16, 17, 18] }];

  for (const input of inputs) {
    const result = numFriendRequests(input.ages);
    console.log(result);
  }
}
main();

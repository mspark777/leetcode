import "@total-typescript/ts-reset";

function longestSquareStreak(nums: number[]): number {
  nums.sort((l, r) => l - r);

  const streakLengths = new Map<number, number>();

  for (const n of nums) {
    const root = Math.trunc(Math.sqrt(n));
    const lenOfRoot = streakLengths.get(root);
    const square = root * root;
    if (square === n && lenOfRoot != null) {
      streakLengths.set(n, lenOfRoot + 1);
    } else {
      streakLengths.set(n, 1);
    }
  }

  let longestLength = 0;
  for (const len of streakLengths.values()) {
    longestLength = Math.max(longestLength, len);
  }

  return longestLength === 1 ? -1 : longestLength;
}

function main(): void {
  const inputs: Array<number[]> = [
    [4, 3, 6, 16, 8, 2],
    [2, 3, 5, 6, 7],
  ];

  for (const nums of inputs) {
    const result = longestSquareStreak(nums);
    console.log(result);
  }
}
main();

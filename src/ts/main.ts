import "@total-typescript/ts-reset";

function kLengthApart(nums: number[], k: number): boolean {
  let left = nums.indexOf(1);
  if (left < 0) {
    return true;
  }

  let zeros = 0;
  for (let right = left + 1; right < nums.length; right += 1) {
    const num = nums[right];
    if (num === 0) {
      zeros += 1;
      continue;
    }

    if (zeros < k) {
      return false;
    }

    left = right;
    zeros = 0;
  }

  return true;
}

interface Input {
  nums: number[];
  k: number;
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 0, 0, 0, 1, 0, 0, 1],
      k: 2,
    },
    {
      nums: [1, 0, 0, 1, 0, 1],
      k: 2,
    },
  ];

  for (const input of inputs) {
    const result = kLengthApart(input.nums, input.k);
    console.log(result);
  }
}
main();

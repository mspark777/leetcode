import "@total-typescript/ts-reset";

function getMinDistance(nums: number[], target: number, start: number): number {
  let left = start;
  let right = start;
  const last = nums.length - 1;
  while (nums[left] !== target && nums[right] !== target) {
    if (left > 0) {
      left -= 1;
    }

    if (right < last) {
      right += 1;
    }
  }

  return Math.max(right - start, start - left);
}

interface Input {
  nums: number[];
  target: number;
  start: number;
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 2, 3, 4, 5],
      target: 5,
      start: 3,
    },
    {
      nums: [1],
      target: 1,
      start: 0,
    },
    {
      nums: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      target: 1,
      start: 0,
    },
  ];

  for (const input of inputs) {
    const result = getMinDistance(input.nums, input.target, input.start);
    console.log(result);
  }
}
main();

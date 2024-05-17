import "@total-typescript/ts-reset";

function bat(arr: boolean[], i: number): boolean {
  return arr.at(i) as boolean;
}

function sat(s: string, i: number): string {
  return s.at(i) as string;
}

function isInterleave(s1: string, s2: string, s3: string): boolean {
  const s1len = s1.length;
  const s2len = s2.length;
  const s3len = s3.length;
  if (s1len + s2len !== s3len) {
    return false;
  }

  if (s1len < s2len) {
    return isInterleave(s2, s1, s3);
  }

  const dp = new Array<boolean>(s2len + 1).fill(false);
  dp[0] = true;
  for (let i = 1; i <= s2len; i += 1) {
    dp[i] = sat(s2, i - 1) === sat(s3, i - 1) && bat(dp, i - 1);
  }

  for (let i = 1; i <= s1len; i += 1) {
    dp[0] = sat(s3, i - 1) === sat(s1, i - 1) && bat(dp, 0);
    for (let j = 1; j <= s2len; j += 1) {
      dp[j] = sat(s3, i + j - 1) === sat(s1, i - 1) && bat(dp, j);
      dp[j] =
        bat(dp, j) || (sat(s3, i + j - 1) === sat(s2, j - 1) && bat(dp, j - 1));
    }
  }

  return bat(dp, -1);
}

function main(): void {
  const inputs: Array<[string, string, string]> = [
    ["aabcc", "dbbca", "aadbbcbcac"],
    ["aabcc", "dbbca", "aadbbbaccc"],
    ["", "", ""],
  ];

  for (const [s1, s2, s3] of inputs) {
    const result = isInterleave(s1, s2, s3);
    console.log(result);
  }
}
main();

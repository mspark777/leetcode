import "@total-typescript/ts-reset";

function isPrefixAndSubfix(str1: string, str2: string): boolean {
  return (
    str2.slice(0, str1.length) === str1 &&
    str2.slice(str2.length - str1.length) === str1
  );
}

function countPrefixSuffixPairs(words: string[]): number {
  let result = 0;

  for (let i = 0; i < words.length; i += 1) {
    for (let j = i + 1; j < words.length; j += 1) {
      if (isPrefixAndSubfix(words[i] as string, words[j] as string)) {
        result += 1;
      }
    }
  }

  return result;
}

function main(): void {
  const inputs: Array<string[]> = [
    ["a", "aba", "ababa", "aa"],
    ["pa", "papa", "ma", "mama"],
    ["abab", "ab"],
  ];

  for (const words of inputs) {
    const result = countPrefixSuffixPairs(words);
    console.log(result);
  }
}
main();

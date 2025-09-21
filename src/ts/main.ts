import "@total-typescript/ts-reset";

function search(word: string): boolean {
  return word.search(/^([a-z]+(-[a-z]+)?[.,!]?|[.,!])$/) > -1;
}

function countValidWords(sentence: string): number {
  return sentence.split(" ").filter(search).length;
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 2, 1],
    },
    {
      nums: [1, 3, 2, 1],
    },
  ];

  for (const input of inputs) {
    const result = getConcatenation(input.nums);
    console.log(result);
  }
}
main();

defmodule Solution do
  @spec longest_str_chain(words :: [String.t]) :: integer
  def longest_str_chain(words) do
    words = Enum.sort_by(words, &String.length/1)
    dp = %{result: 0}
    solve(words, dp)
  end

  def solve([word | words], dp) do
    dp = loopword(0, String.length(word), 1, word, dp)
    solve(words, dp)
  end

  def solve([], dp) do
    dp.result
  end

  def loopword(i, word_len, cur_len, word, dp) when i < word_len do
    predecessor = "#{substring(word, 0, i - 1)}#{substring(word, i + 1, word_len)}"
    pre_len = Map.get(dp, predecessor, -1)
    cur_len = max(cur_len, pre_len + 1)
    loopword(i + 1, word_len, cur_len, word, dp)
  end

  def loopword(_, _, cur_len, word, dp) do
    dp = Map.put(dp, word, cur_len)
    Map.put(dp, :result, max(dp.result, cur_len))
  end

  def substring(word, i, j) do
    if j < 0 do
      ""
    else
      String.slice(word, i..j)
    end
  end

  def main() do
    inputs = [
      ["a", "b", "ba", "bca", "bda", "bdca"],
      ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"],
      ["abcd", "dbqca"]
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = longest_str_chain(input)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Solution.main


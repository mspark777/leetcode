defmodule Leetcode do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[p, s] | inputs]) do
    result = Solution.word_pattern(p, s)
    IO.puts(result)
    main(inputs)
  end

  def main([]) do
  end

  def main do
    main([
      ["abba", "dog cat cat dog"],
      ["abba", "dog cat cat fish"],
      ["aaaa", "dog cat cat dog"],
      ["abba", "dog dog dog dog"]
    ])
  end
end

Leetcode.main()

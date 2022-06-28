defmodule Leetcode do
  def main() do
    inputs = [
      %{ s: "aab" },
      %{ s: "aaabbbcc" },
      %{ s: "ceabaacb" }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.min_deletions(input.s)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()

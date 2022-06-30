defmodule Leetcode do
  def main() do
    inputs = [
      %{ nums: [1, 2, 3] },
      %{ nums: [1, 10, 2, 9] }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.min_moves2(input.nums)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()

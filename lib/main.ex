defmodule Leetcode do
  import Solution

  def main() do
    inputs = [
      %{ nums: [3, 2, 1, 5, 6, 4], k: 2 },
      %{ nums: [3, 2, 3, 1, 2, 4, 5, 5, 6], k: 4 }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = find_kth_largest(input.nums, input.k)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()

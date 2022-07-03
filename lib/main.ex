defmodule Leetcode do
  def main() do
    inputs = [
        %{
          nums: [1, 7, 4, 9, 2, 5]
        },
        %{
          nums: [1, 17, 5, 10, 13, 15, 10, 5, 16, 8]
        },
        %{
          nums: [1, 2, 3, 4, 5, 6, 7, 8, 9]
        }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.wiggle_max_length(input.nums)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()

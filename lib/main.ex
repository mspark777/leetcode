defmodule Solution do
  @spec max_subarray_sum_circular(nums :: [integer]) :: integer
  def max_subarray_sum_circular(nums), do: loop(nums, 0, 0, 0, hd(nums), hd(nums))

  defp loop([], _, _, sum, minsum, maxsum) when sum == minsum, do: maxsum
  defp loop([], _, _, sum, minsum, maxsum), do: max(maxsum, sum - minsum)

  defp loop([num | nums], curmax, curmin, sum, minsum, maxsum) do
    curmax = max(curmax, 0) + num
    curmin = min(curmin, 0) + num
    maxsum = max(maxsum, curmax)
    minsum = min(minsum, curmin)
    loop(nums, curmax, curmin, sum + num, minsum, maxsum)
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([nums | remains]) do
    result = Solution.max_subarray_sum_circular(nums)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [1, -2, 3, -2],
      [5, -3, 5],
      [-3, -2, -3]
    ])
  end
end

Main.main()

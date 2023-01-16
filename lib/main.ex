defmodule Solution do
  @spec is_perfect_square(num :: integer) :: boolean
  def is_perfect_square(num), do: loop(1, div(num, 2), num)

  @spec loop(left :: integer, right :: integer, num :: integer) :: boolean
  defp loop(left, right, num)

  defp loop(left, right, num) when left <= right do
    mid = div(left + right, 2)
    square = mid * mid

    cond do
      num < square -> loop(left, mid - 1, num)
      num > square -> loop(mid + 1, right, num)
      true -> true
    end
  end

  defp loop(_, _, num), do: num == 1
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([num | remains]) do
    result = Solution.is_perfect_square(num)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      16,
      14,
      1
    ])
  end
end

Main.main()

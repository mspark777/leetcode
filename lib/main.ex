defmodule Solution do
  @spec count_odds(low :: integer, high :: integer) :: integer
  def count_odds(low, high)
  def count_odds(low, high) when rem(low, 2) == 0, do: count_odds(low + 1, high)
  def count_odds(low, high) when low > high, do: 0
  def count_odds(low, high), do: div(high - low, 2) + 1
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[low, high] | remains]) do
    result = Solution.count_odds(low, high)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [3, 7],
      [8, 10]
    ])
  end
end

Main.main()

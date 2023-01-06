defmodule Solution do
  @spec max_ice_cream(costs :: [integer], coins :: integer) :: integer
  def max_ice_cream(costs, coins) do
    costs |> Enum.sort() |> loop(coins, 0)
  end

  defp loop([], _, result), do: result

  defp loop([cost | costs], coins, result) when coins >= cost do
    loop(costs, coins - cost, result + 1)
  end

  defp loop(_, _, result), do: result
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([input | remains]) do
    result = Solution.max_ice_cream(input.costs, input.coins)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      %{costs: [1, 3, 2, 4, 1], coins: 7},
      %{costs: [10, 6, 8, 7, 7, 8], coins: 5},
      %{costs: [1, 6, 3, 1, 2, 5], coins: 20}
    ])
  end
end

Main.main()

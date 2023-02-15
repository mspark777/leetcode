defmodule Solution do
  @spec add_to_array_form(num :: [integer], k :: integer) :: [integer]
  def add_to_array_form(num, k) do
    num |> Enum.reverse() |> loop(k, [])
  end

  defp loop([num | nums], cur, result) do
    sum = cur + num
    loop(nums, div(sum, 10), [rem(sum, 10) | result])
  end

  defp loop([] = nums, cur, result) when cur > 0 do
    loop(nums, div(cur, 10), [rem(cur, 10) | result])
  end

  defp loop([], _, result), do: result
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([input | remains]) do
    result = Solution.add_to_array_form(input.num, input.k)

    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      %{num: [1, 2, 0, 0], k: 34},
      %{num: [2, 7, 4], k: 181},
      %{num: [2, 1, 5], k: 806},
      %{num: [1, 2, 6, 3, 0, 7, 1, 7, 1, 9, 7, 5, 6, 6, 4, 4, 0, 0, 6, 3], k: 516},
      %{num: [0], k: 10000}
    ])
  end
end

Main.main()

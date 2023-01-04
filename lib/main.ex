defmodule Solution do
  @spec minimum_rounds(tasks :: [integer]) :: integer
  def minimum_rounds(tasks) do
    tasks |> count_frequencies(%{}) |> Map.values() |> calc_result(0)
  end

  defp calc_result([count | _], _) when count == 1, do: -1
  defp calc_result([], result), do: result

  defp calc_result([count | counts], result) do
    if rem(count, 3) == 0 do
      calc_result(counts, result + div(count, 3))
    else
      calc_result(counts, result + div(count, 3) + 1)
    end
  end

  defp count_frequencies([], counts), do: counts

  defp count_frequencies([task | tasks], counts) do
    count = Map.get(counts, task, 0)
    count_frequencies(tasks, Map.put(counts, task, count + 1))
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([tasks | remains]) do
    result = Solution.minimum_rounds(tasks)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [2, 2, 3, 3, 2, 4, 4, 4, 4, 4],
      [2, 3, 3]
    ])
  end
end

Main.main()

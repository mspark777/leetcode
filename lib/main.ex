defmodule Solution do
  @spec find_min_arrow_shots(points :: [[integer]]) :: integer
  def find_min_arrow_shots(points) do
    [cur | remains] = points |> Enum.sort_by(&Enum.at(&1, 1))
    loop(remains, cur, 1)
  end

  defp loop([], _, result), do: result

  defp loop([[cl, _] = cur | points], [_, pr] = prev, result) do
    if cl > pr do
      loop(points, cur, result + 1)
    else
      loop(points, prev, result)
    end
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([points | remains]) do
    result = Solution.find_min_arrow_shots(points)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [[10, 16], [2, 8], [1, 6], [7, 12]],
      [[1, 2], [3, 4], [5, 6], [7, 8]],
      [[1, 2], [2, 3], [3, 4], [4, 5]]
    ])
  end
end

Main.main()

defmodule Solution do
  @spec find_judge(n :: integer, trust :: [[integer]]) :: integer
  def find_judge(n, trust)

  def find_judge(1, []), do: 1

  def find_judge(n, trust) do
    trust |> count_trust(%{}) |> Map.to_list() |> check(n - 1)
  end

  defp count_trust([[from, to] | trust], counts) do
    f = Map.get(counts, from, 0) - 1
    t = Map.get(counts, to, 0) + 1
    count_trust(trust, counts |> Map.put(from, f) |> Map.put(to, t))
  end

  defp count_trust([], counts), do: counts

  defp check(counts, judge)
  defp check([{person, count} | _], judge) when count == judge, do: person
  defp check([_ | counts], judge), do: check(counts, judge)
  defp check([], _), do: -1
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([%{n: n, trust: trust} | remains]) do
    result = Solution.find_judge(n, trust)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      %{n: 2, trust: [[1, 2]]},
      %{n: 3, trust: [[1, 3], [2, 3]]},
      %{n: 3, trust: [[1, 3], [2, 3], [3, 1]]},
      %{n: 1, trust: []}
    ])
  end
end

Main.main()

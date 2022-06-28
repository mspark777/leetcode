defmodule Solution do
  @spec min_deletions(s :: String.t) :: integer
  def min_deletions(s) do
    s |> String.to_charlist |> count_frequency(%{}, 0)
  end

  @spec count_frequency(chars :: list(char()), map :: %{char() => integer()}, max_f :: integer()) :: integer()
  def count_frequency([ch | chars], map, max_f) do
    f = Map.get(map, ch, 0)
    count_frequency(chars, Map.put(map, ch, f + 1), max_f + 1)
  end

  def count_frequency([], map, max_f) do
    map |> Map.values |> Enum.sort(&(&1 > &2)) |> solve(max_f, 0)
  end

  @spec count_frequency(frequency :: list(integer()), max_f :: integer(), result :: integer()) :: integer()
  def solve([f | frequency], max_f, result) when f > max_f do
    result = result + (f - max_f)
    max_f = max(0, max_f - 1)
    solve(frequency, max_f, result)
  end

  def solve([f | frequency], _, result)  do
    max_f = max(0, f - 1)
    solve(frequency, max_f, result)
  end

  def solve([], _, result)  do
    result
  end
end

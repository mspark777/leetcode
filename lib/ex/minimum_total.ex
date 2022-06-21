defmodule MinimumTotal do
  @spec minimum_total(triangle :: [[integer]]) :: integer
  def minimum_total(triangle) do
    depth = Enum.count(triangle)
    triangle |> Enum.map(&list_to_map/1) |> list_to_map |> solve(depth - 2)
  end

  def solve(triangle, i) when i >= 0 do
    triangle = sum_minimum(triangle, i, 0)
    solve(triangle, i - 1)
  end

  def solve(triangle, _) do
    Map.get(Map.get(triangle, 0), 0)
  end

  def sum_minimum(triangle, i, j) do
    rows = Map.get(triangle, i)
    if Map.has_key?(rows, j) do
      belows = Map.get(triangle, i + 1)
      sum = Map.get(rows, j) + min(Map.get(belows, j), Map.get(belows, j + 1))
      rows = Map.put(rows, j, sum)
      triangle = Map.put(triangle, i, rows)
      sum_minimum(triangle, i, j + 1)
    else
      triangle
    end
  end

  def list_to_map(list) do
    list |> Enum.with_index() |> Map.new(fn {v, i} -> {i, v} end)
  end

end

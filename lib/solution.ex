defmodule Solution do
  @spec max_area(h :: integer, w :: integer, horizontal_cuts :: [integer], vertical_cuts :: [integer]) :: integer
  def max_area(h, w, horizontal_cuts, vertical_cuts) do
    max_h = ([h] ++ horizontal_cuts) |> Enum.sort |> max_distance(0, 0)
    max_w = ([w] ++ vertical_cuts) |> Enum.sort |> max_distance(0, 0)
    rem(max_w * max_h, 1000000007)
  end

  def max_distance([back | remains], front, result) do
    max_distance(remains, back, max(result, back - front))
  end

  def max_distance([], _, result), do: result
end

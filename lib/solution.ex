defmodule Solution do
  @spec maximum_units(box_types :: [[integer]], truck_size :: integer) :: integer
  def maximum_units(box_types, truck_size) do
    box_types
    |> Enum.sort(fn [_, u0], [_, u1] -> u0 > u1 end)
    |> solve(truck_size, 0)
  end

  def solve([[count, units] | box_types], truck_size, result) do
    cnt = min(truck_size, count)
    solve(box_types, truck_size - cnt, result + units * cnt)
  end

  def solve(_, truck_size, result) when truck_size <= 0, do: result
  def solve([], _, result), do: result
end

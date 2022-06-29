defmodule Solution do
  @spec reconstruct_queue(people :: [[integer]]) :: [[integer]]
  def reconstruct_queue(people) do
    people
    |> Enum.sort(fn [hi, ki], [hj, kj] -> if hi == hj, do: ki < kj, else: hi > hj end)
    |> solve([])
  end

  def solve([[_, h] = person | people], result) do
    solve(people, List.insert_at(result, h, person))
  end
  def solve([], result), do: result
end

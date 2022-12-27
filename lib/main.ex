defmodule Solution do
  @spec maximum_bags(capacity :: [integer], rocks :: [integer], additional_rocks :: integer) ::
          integer
  def maximum_bags(capacity, rocks, additional_rocks) do
    Enum.zip(capacity, rocks)
    |> Enum.map(fn {c, r} -> c - r end)
    |> Enum.sort()
    |> loop(additional_rocks, 0)
  end

  defp loop([remain | remains], additional_rocks, result)
       when additional_rocks >= remain do
    loop(remains, additional_rocks - remain, result + 1)
  end

  defp loop([], _, result), do: result
  defp loop(_, _, result), do: result
end

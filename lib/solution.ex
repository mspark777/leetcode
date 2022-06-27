defmodule Solution do
  @spec min_partitions(n:: String.t()):: integer
  def min_partitions(n) do
    n |> String.graphemes |> Enum.max |> String.to_integer
  end
end

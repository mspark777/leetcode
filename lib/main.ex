defmodule Solution do
  @spec min_set_size(arr :: [integer]) :: integer
  def min_set_size(arr) do
    half = length(arr) |> div(2)

    arr
    |> Enum.frequencies()
    |> Map.values()
    |> Enum.sort(&(&1 >= &2))
    |> Enum.with_index()
    |> Enum.reduce_while(0, &reduce(&1, &2, half))
  end

  @spec reduce(w :: {integer, integer}, acc :: integer, half :: integer) :: {atom, integer}
  defp reduce({freq, idx}, acc, half) do
    deleted = acc + freq

    if deleted >= half, do: {:halt, idx + 1}, else: {:cont, deleted}
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        arr: [3, 3, 3, 3, 5, 5, 5, 2, 2, 7]
      },
      %{
        arr: [7, 7, 7, 7, 7, 7]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    arr = input.arr
    result = Solution.min_set_size(arr)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()

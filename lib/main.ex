defmodule Solution do
  use Bitwise
  @spec valid_utf8(data :: [integer]) :: boolean
  def valid_utf8([]), do: false
  def valid_utf8(data), do: data |> loop(0)

  @spec loop(data :: [integer], bytes :: integer) :: boolean
  defp loop([i | data], bytes) when bytes == 0 do
    new_bytes = calc_bytes(i, 128, 0)

    cond do
      new_bytes == 0 -> loop(data, 0)
      new_bytes > 4 -> false
      new_bytes == 1 -> false
      true -> loop(data, new_bytes - 1)
    end
  end

  defp loop([i | _data], _bytes) when (i &&& 128) == 0 or (i &&& 64) != 0, do: false
  defp loop([_i | data], bytes), do: loop(data, bytes - 1)
  defp loop([], bytes), do: bytes == 0

  @spec calc_bytes(i :: integer, mask :: integer, bytes :: integer) :: integer
  defp calc_bytes(i, mask, bytes) when (mask &&& i) != 0, do: calc_bytes(i, mask >>> 1, bytes + 1)
  defp calc_bytes(_i, _mask, bytes), do: bytes
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        data: [197, 130, 1]
      },
      %{
        data: [235, 140, 4]
      },
      %{
        data: [240, 162, 138, 147]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([%{data: data} | remains]) do
    result = Solution.valid_utf8(data)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()

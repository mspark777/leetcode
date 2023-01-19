defmodule Solution do
  use Bitwise

  @spec read_binary_watch(turned_on :: integer) :: [String.t()]
  def read_binary_watch(turned_on) do
    loop1(turned_on, 0, []) |> Enum.reverse()
  end

  defp loop1(turned_on, h, result) when h < 12 do
    loop1(turned_on, h + 1, loop2(turned_on, h, 0, result))
  end

  defp loop1(_, _, result), do: result

  defp loop2(turned_on, h, m, result) when m < 60 do
    num = h <<< 6 ||| m
    ones = Enum.sum(for <<bit::1 <- :binary.encode_unsigned(num)>>, do: bit)

    if ones == turned_on do
      if m >= 10 do
        loop2(turned_on, h, m + 1, ["#{h}:#{m}" | result])
      else
        loop2(turned_on, h, m + 1, ["#{h}:0#{m}" | result])
      end
    else
      loop2(turned_on, h, m + 1, result)
    end
  end

  defp loop2(_, _, _, result), do: result
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([turned_on | remains]) do
    result = Solution.read_binary_watch(turned_on)

    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      1,
      9
    ])
  end
end

Main.main()

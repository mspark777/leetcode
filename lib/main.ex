defmodule Solution do
  use Bitwise

  @spec find_the_difference(s :: String.t(), t :: String.t()) :: char
  def find_the_difference(s, t) do
    t
    |> String.to_charlist()
    |> Enum.reduce(sreduce(s), fn acc, cur -> Bitwise.bxor(acc, cur) end)
  end

  @spec sreduce(s :: String.t()) :: char
  defp sreduce(s) do
    s
    |> String.to_charlist()
    |> Enum.reduce(0, fn acc, cur -> Bitwise.bxor(acc, cur) end)
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[s, t] | remains]) do
    result = Solution.find_the_difference(s, t)

    IO.puts([result] |> to_string())
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      ["abcd", "abcde"],
      ["", "y"]
    ])
  end
end

Main.main()

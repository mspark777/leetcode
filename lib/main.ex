defmodule Solution do
  @spec min_flips_mono_incr(s :: String.t()) :: integer
  def min_flips_mono_incr(s) do
    s
    |> String.to_charlist()
    |> Enum.reduce({0, 0}, &reducefunc/2)
    |> elem(0)
  end

  defp reducefunc(c, {result, num}) do
    case c do
      ?0 -> {min(result + 1, num), num}
      _ -> {result, num + 1}
    end
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([s | remains]) do
    result = Solution.min_flips_mono_incr(s)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      "00110",
      "010110",
      "00011000"
    ])
  end
end

Main.main()

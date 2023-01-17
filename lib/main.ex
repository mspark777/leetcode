defmodule Solution do
  @spec is_subsequence(s :: String.t(), t :: String.t()) :: boolean
  def is_subsequence(s, t), do: loop(String.to_charlist(s), String.to_charlist(t))

  defp loop([s | schars], [t | tchars]) when s == t, do: loop(schars, tchars)
  defp loop(schars, [_ | tchars]), do: loop(schars, tchars)
  defp loop([], _), do: true
  defp loop(_, []), do: false
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[s, t] | remains]) do
    result = Solution.is_subsequence(s, t)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      ["abc", "ahbgdc"],
      ["axc", "ahbgdc"],
      ["", ""],
      ["", "ahbgdc"]
    ])
  end
end

Main.main()

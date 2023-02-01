defmodule Solution do
  @spec gcd_of_strings(str1 :: String.t(), str2 :: String.t()) :: String.t()
  def gcd_of_strings(str1, str2)

  def gcd_of_strings(str1, str2) when str1 <> str2 != str2 <> str1, do: ""

  def gcd_of_strings(str1, str2) do
    gcdlen = gcd(String.length(str1), String.length(str2))

    String.slice(str1, 0, gcdlen)
  end

  @spec gcd(x :: integer, y :: integer) :: integer
  def gcd(x, y)
  def gcd(x, 0), do: x
  def gcd(x, y), do: gcd(y, rem(x, y))
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[str1, str2] | remains]) do
    result = Solution.gcd_of_strings(str1, str2)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      ["ABCABC", "ABC"],
      ["ABABAB", "ABAB"],
      ["LEET", "CODE"]
    ])
  end
end

Main.main()

defmodule Solution do
  @spec is_palindrome(s :: String.t) :: boolean
  def is_palindrome(s) do
    s0 = s
      |> String.downcase
      |> String.to_charlist
      |> Enum.filter(&isalnum/1)
      |> to_string

    s0 == String.reverse(s0)
  end

  @spec isalnum(ch :: char) :: boolean
  def isalnum(ch) do
    ((?a <= ch) and (ch <= ?z)) or ((?0 <= ch) and (ch <= ?9))
  end
end

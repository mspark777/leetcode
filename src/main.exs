defmodule Solution do
  @spec longest_palindrome(s :: String.t) :: String.t
  def longest_palindrome(s) do
    arr = s |> String.to_charlist |> List.to_tuple
    slen = tuple_size(arr)
    case slen do
      1 -> s
      2 -> twolen(arr)
      _ -> solve(s, arr, slen, 1, 0, 0)
    end
  end

  def solve(s, arr, slen, i, start, max_len) when i < (slen - 1) do
    low = calc_low(arr, i, i - 1)
    high = calc_high(arr, i, slen, i + 1)
    {curstart, curlen} = calc_range(arr, slen, low, high)
    if max_len < curlen do
      solve(s, arr, slen, i + 1, curstart, curlen)
    else
      solve(s, arr, slen, i + 1, start, max_len)
    end
  end

  def solve(s, _, _, _, start, max_len) do
    substring(s, start, start + max_len - 1)
  end

  def calc_range(arr, slen, low, high) when low > -1 and high < slen do
    if elem(arr, low) == elem(arr, high) do
      calc_range(arr, slen, low - 1, high + 1)
    else
      {low + 1, high - low - 1}
    end
  end

  def calc_range(_, _, low, high) do
    {low + 1, high - low - 1}
  end

  def calc_high(arr, i, slen, high) when high < slen do
    if elem(arr, high) == elem(arr, i) do
      calc_high(arr, i, slen, high + 1)
    else
      high
    end
  end

  def calc_high(_, _, _, high) do
    high
  end

  def calc_low(arr, i, low) when low > -1 do
    if elem(arr, low) == elem(arr, i) do
      calc_low(arr, i, low - 1)
    else
      low
    end
  end

  def calc_low(_, _, low) do
    low
  end

  def twolen(arr) do
    left = elem(arr, 0)
    right = elem(arr, 1)
    if left == right do
      List.to_string([left, right])
    else
      List.to_string([left])
    end
  end

  def substring(s, i, j) do
    if j < 0 do
      ""
    else
      String.slice(s, i..j)
    end
  end

  def main() do
    inputs = [
"babad", "cbbd", "ac", "aba", "bb",
"zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = longest_palindrome(input)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Solution.main


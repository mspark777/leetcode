defmodule Solution do
  @spec longest_palindrome(s :: String.t) :: String.t
  def longest_palindrome(s) do
    slen = String.length(s)
    case slen do
      1 -> s
      2 -> twolen(s)
      _ -> s
        |> String.graphemes
        |> Enum.with_index()
        |> Enum.map(fn {k,v}->{v,k} end)
        |> Map.new
        |> Map.merge(%{s: s, maxlen: 0, start: 0, slen: slen, i: 1})
        |> solve
    end
  end

  def solve(map) do
    i = map.i
    slen = map.slen
    if i < (slen - 1) do
      map = Map.merge(map, %{
        low: i - 1,
        high: i + 1,
        curlen: 0
      })

      map = updatelow(map)
      map = updatehigh(map)
      map = updatecurlen(map)

      curlen = map.curlen
      maxlen = map.maxlen
      map = Map.put(map, :i, i + 1)
      if maxlen < curlen do
        map = Map.merge(map, %{
          maxlen: curlen,
          start: map.low + 1
        })
        solve(map)
      else
        solve(map)
      end
    else
      si = map.start
      sj = map.maxlen + si - 1
      substring(map.s, si,  sj)
    end
  end

  def updatecurlen(map) do
    low = map.low
    high = map.high
    slen = map.slen
    if low > -1 and high < slen and map[low] == map[high] do
      map = Map.merge(map, %{
        low: low - 1,
        high: high + 1
      })
      updatecurlen(map)
    else
      Map.put(map, :curlen, map.high - map.low - 1)
    end
  end

  def updatelow(map) do
    low = map.low
    i = map.i
    if low > -1 and map[low] == map[i] do
      map = Map.put(map, :low, low - 1)
      updatelow(map)
    else
      map
    end
  end

  def updatehigh(map) do
    high = map.high
    slen = map.slen
    i = map.i
    if high < slen and map[high] == map[i] do
      map = Map.put(map, :high, high + 1)
      updatehigh(map)
    else
      map
    end
  end

  def twolen(s) do
    if String.at(s, 0) == String.at(s, 1) do
      s
    else
      substring(s, 0, 0)
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


defmodule MinDistance do
  @spec min_distance(word1 :: String.t, word2 :: String.t) :: integer
  def min_distance(word1, word2) do
    list1 = String.graphemes(word1)
    list2 = String.graphemes(word2)
    len2 = String.length(word2)
    dp = list2 |> Enum.with_index() |> Map.new(fn {_, i} -> {i, 0} end)
    dp = Map.merge(dp, %{
      len2 => 0,
      len1: String.length(word1),
      len2: len2
    })

    solve(list1, list2, 0, 0, dp, list2)
  end

  def solve([ch1 | chs1], [ch2 | chs2], j, prev, dp, list2) when ch1 == ch2 do
    val = max(dp[j + 1], prev + 1)
    prev = dp[j + 1]
    dp = Map.put(dp, j + 1, val)
    solve([ch1 | chs1], chs2, j + 1, prev, dp, list2)
  end

  def solve([ch1 | chs1], [ch2 | chs2], j, _, dp, list2) when ch1 != ch2 do
    val = max(dp[j + 1], dp[j])
    prev = dp[j + 1]
    dp = Map.put(dp, j + 1, val)
    solve([ch1 | chs1], chs2, j + 1, prev, dp, list2)
  end

  def solve([_ | chs1], [], _, _, dp, list2) do
    prev = dp[0]
    solve(chs1, list2, 0, prev, dp, list2)
  end

  def solve([], _, _, _, dp, _) do
    dp.len1 + dp.len2 - (2 * dp[dp.len2])
  end
end

defmodule MinDistance1 do
  @spec min_distance(word1 :: String.t, word2 :: String.t) :: integer
  def min_distance(word1, word2) do
    list1 = word1 |> String.graphemes |> Enum.with_index()
    list2 = word2 |> String.graphemes |> Enum.with_index()
    dp = %{0 => -0, prev: 0, list2: list2}

    solve(list1, list2, dp)
  end

  def solve([{ch1, i} | _] = list1, [{ch2, j} | chs2], dp) when ch1 == ch2 do
    dp = set_word_lens(dp, i, j)
    next = j + 1
    dp = set_default(dp, next)
    val = max(dp[next], dp.prev + 1)
    dp = update_prev(dp, next, val)
    solve(list1, chs2, dp)
  end

  def solve([{ch1, i} | _] = list1, [{ch2, j} | chs2], dp) when ch1 != ch2 do
    dp = set_word_lens(dp, i, j)
    next = j + 1
    dp = set_default(dp, j)
    dp = set_default(dp, next)
    val = max(dp[next], dp[j])
    dp = update_prev(dp, next, val)
    solve(list1, chs2, dp)
  end

  def solve([_ | chs1], [], dp) do
    dp = Map.put(dp, :prev, dp[0])
    solve(chs1, dp.list2, dp)
  end

  def solve([], _, dp) do
    dp.len1 + dp.len2 - (2 * dp[dp.len2])
  end

  def update_prev(dp, k, v) do
    Map.merge(dp, %{
      k => v,
      prev: dp[k]
    })
  end

  def set_word_lens(dp, i, j) do
    Map.merge(dp, %{ len1: i + 1, len2: j + 1 })
  end

  def set_default(dp, k) do
    if Map.has_key?(dp, k) do
      dp
    else
      Map.put(dp, k, 0)
    end
  end

end

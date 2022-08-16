defmodule Solution do
  @spec first_uniq_char(s :: String.t()) :: integer
  def first_uniq_char(s) do
    chars = String.to_charlist(s)
    memo = count(chars, %{})
    check(chars, memo, 0)
  end

  @spec count(char :: [char], memo :: %{char => integer}) :: %{char => integer}
  defp count([ch | chars], memo) do
    cnt = Map.get(memo, ch, 0)
    count(chars, Map.put(memo, ch, cnt + 1))
  end

  defp count([], memo), do: memo

  @spec check(char :: [char], memo :: %{char => integer}, i :: integer) :: integer
  defp check([ch | chars], memo, i) do
    case Map.get(memo, ch) do
      1 -> i
      _ -> check(chars, memo, i + 1)
    end
  end

  defp check([], _, _), do: -1
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        s: "leetcode"
      },
      %{
        s: "loveleetcode"
      },
      %{
        s: "aabb"
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    s = input.s
    result = Solution.first_uniq_char(s)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()

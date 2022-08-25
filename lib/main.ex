defmodule Solution do
  @spec can_construct(ransom_note :: String.t(), magazine :: String.t()) :: boolean
  def can_construct(ransom_note, magazine) do
    counts = magazine |> String.to_charlist() |> create_counts(%{})
    ransom_note |> String.to_charlist() |> check_counts(counts)
  end

  defp create_counts([ch | chars], counts) do
    cnt = Map.get(counts, ch, 0)
    create_counts(chars, Map.put(counts, ch, cnt + 1))
  end

  defp create_counts([], counts), do: counts

  defp check_counts([ch | chars], counts) do
    cnt = Map.get(counts, ch, 0)

    if cnt < 1 do
      false
    else
      check_counts(chars, Map.put(counts, ch, cnt - 1))
    end
  end

  defp check_counts([], _), do: true
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        ransom_note: "a",
        magazine: "b"
      },
      %{
        ransom_note: "aa",
        magazine: "ab"
      },
      %{
        ransom_note: "aa",
        magazine: "aab"
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    ransom_note = input.ransom_note
    magazine = input.magazine
    result = Solution.can_construct(ransom_note, magazine)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()

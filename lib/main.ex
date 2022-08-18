defmodule Solution do
  @type chlist :: [{char, integer}]
  @type index_map :: %{optional(char) => integer}

  @spec is_isomorphic(s :: String.t(), t :: String.t()) :: boolean
  def is_isomorphic(s, t), do: transform(s) == transform(t)

  @spec transform(s :: String.t()) :: String.t()
  defp transform(s) do
    s
    |> String.to_charlist()
    |> Enum.with_index()
    |> transform(%{}, [])
  end

  @spec transform(s :: chlist, mapping :: index_map, result :: [integer]) :: String.t()
  defp transform([{ch, idx} | s], mapping, result) do
    if Map.has_key?(mapping, ch) do
      i = Map.get(mapping, ch)
      transform(s, mapping, [i | result])
    else
      transform(s, Map.put(mapping, ch, idx), [idx | result])
    end
  end

  defp transform([], _, result), do: Enum.join(result, " ")
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        s: "egg",
        t: "add"
      },
      %{
        s: "foo",
        t: "bar"
      },
      %{
        s: "paper",
        t: "title"
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    s = input.s
    t = input.t
    result = Solution.is_isomorphic(s, t)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()

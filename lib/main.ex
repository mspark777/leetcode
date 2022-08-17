defmodule Solution do
  @spec title_to_number(column_title :: String.t()) :: integer
  def title_to_number(column_title) do
    column_title
    |> String.to_charlist()
    |> Enum.map(&(&1 - ?A + 1))
    |> Enum.reduce(&(&2 * 26 + &1))
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        column_title: "A"
      },
      %{
        column_title: "AB"
      },
      %{
        column_title: "ZY"
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    column_title = input.column_title
    result = Solution.title_to_number(column_title)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()

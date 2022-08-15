defmodule Solution do
  @spec convert_to_title(column_number :: integer) :: String.t()
  def convert_to_title(column_number), do: convert_to_title(column_number, [])

  @spec convert_to_title(n :: integer, result :: [char]) :: String.t()
  def convert_to_title(n, result) when n > 0 do
    n = n - 1
    base = 26
    temp = ?A + rem(n, base)
    convert_to_title(div(n, base), [temp | result])
  end

  def convert_to_title(_, result), do: to_string(result)
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        column_number: 1
      },
      %{
        column_number: 28
      },
      %{
        column_number: 701
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.column_number
    result = Solution.convert_to_title(n)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()

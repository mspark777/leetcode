defmodule Solution do
  @spec is_happy(n :: integer) :: boolean
  def is_happy(n), do: is_happy(n, get_next(n, 0))

  @spec is_happy(slow :: integer, fast :: integer) :: boolean
  defp is_happy(slow, fast) when fast != 1 and slow != fast do
    nslow = get_next(slow, 0)
    nfast = fast |> get_next(0) |> get_next(0)

    is_happy(nslow, nfast)
  end

  defp is_happy(_, fast), do: fast == 1

  @spec get_next(n :: integer, result :: integer) :: integer
  defp get_next(n, result) when n > 0 do
    d = rem(n, 10)
    get_next(div(n, 10), result + d * d)
  end

  defp get_next(_, result), do: result
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 19
      },
      %{
        n: 2
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    result = Solution.is_happy(n)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()

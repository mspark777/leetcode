defmodule Solution do
  @spec is_ugly(n :: integer) :: boolean
  def is_ugly(n) when n <= 0, do: false
  def is_ugly(n) when n == 1, do: true
  def is_ugly(n) when rem(n, 2) == 0, do: n |> div(2) |> is_ugly()
  def is_ugly(n) when rem(n, 3) == 0, do: n |> div(3) |> is_ugly()
  def is_ugly(n) when rem(n, 5) == 0, do: n |> div(5) |> is_ugly()
  def is_ugly(_n), do: false
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 6
      },
      %{
        n: 1
      },
      %{
        n: 14
      },
      %{
        n: -2_147_483_648
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    result = Solution.is_ugly(n)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()

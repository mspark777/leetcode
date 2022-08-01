defmodule Solution do
  @spec unique_paths(m :: integer, n :: integer) :: integer
  def unique_paths(m, n) do
    total = m + n - 2
    r = min(n, m) - 1

    unique_paths(total, r, 1, 1)
  end

  @spec unique_paths(
    total :: integer,
    r :: integer,
    i :: integer,
    steps :: integer
  ) :: integer
  defp unique_paths(total, r, i, steps) when i <= r do
    unique_paths(total - 1, r, i + 1, div(steps * total, i))
  end

  defp unique_paths(_, _, _, steps), do: steps
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        m: 3,
        n: 7
      },
      %{
        m: 3,
        n: 2
      },
      %{
        m: 51,
        n: 9
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    m = input.m
    n = input.n
    result = Solution.unique_paths(m, n)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()

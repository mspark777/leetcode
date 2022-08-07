defmodule Solution do
  @spec count_vowel_permutation(n :: integer) :: integer
  def count_vowel_permutation(n) do
    count_vowel_permutation(1000000007, 1, 1, 1, 1, 1, n - 1)
  end

  @spec count_vowel_permutation(
    modulo :: integer,
    a :: integer,
    e :: integer,
    i :: integer,
    o :: integer,
    u :: integer,
    n :: integer
  ) :: integer
  def count_vowel_permutation(modulo, a, e, i, o, u, n) when n > 0 do
    nexta = e + i + u
    nexte = a + i
    nexti = e + o
    nexto = i
    nextu = i + o
    count_vowel_permutation(
      modulo,
      rem(nexta, modulo),
      rem(nexte, modulo),
      rem(nexti, modulo),
      rem(nexto, modulo),
      rem(nextu, modulo),
      n - 1
    )
  end

  def count_vowel_permutation(modulo, a, e, i, o, u, _), do: rem(a + e + i + o + u, modulo)
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        n: 1
      },
      %{
        n: 2
      },
      %{
        n: 5
      },
      %{
        n: 144
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    result = Solution.count_vowel_permutation(n)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()

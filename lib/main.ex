defmodule Solution do
  @spec mirror_reflection(p :: integer, q :: integer) :: integer
  def mirror_reflection(p, q) when (rem(p, 2) + rem(q, 2)) == 0 do
    mirror_reflection(div(p, 2), div(q, 2))
  end

  def mirror_reflection(p, q), do: rem(q, 2) - rem(p, 2) + 1
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        p: 2,
        q: 1
      },
      %{
        p: 3,
        q: 1
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    p = input.p
    q = input.q
    result = Solution.mirror_reflection(p, q)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        s: "anagram",
        t: "nagaram",
      },
      %{
        s: "rat",
        t: "car",
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    result = Solution.is_anagram(input.s, input.t)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()

defmodule Main do
  def main() do
    inputs = [
      %{
        s: "A man, a plan, a canal: Panama",
      },
      %{
        s: "race a car",
      },
      %{
        s: " ",
      },
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.is_palindrome(input.s)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end
Main.main()

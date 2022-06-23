defmodule Leetcode do
  def get_json(filename) do
    with {:ok, body} <- File.read(filename),
         {:ok, json} <- Poison.decode(body), do: {:ok, json}
  end

  def main() do
    {:ok, data} = get_json('./src/data.json')
    inputs = [
      %{ courses: [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]] },
      %{ courses: [[1, 2]] },
      %{ courses: [[3, 2], [4, 3]] },
      %{ courses: [[5, 15], [3, 19], [6, 7], [2, 10], [5, 16], [8, 14], [10, 11], [2, 19], ]},
      %{ courses: data["courses"]}
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.schedule_course(input.courses)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()

defmodule ListNode do
  @type t :: %__MODULE__{
          val: integer,
          next: ListNode.t() | nil
        }
  defstruct val: 0, next: nil
end

defmodule Solution do
  @spec is_palindrome(head :: ListNode.t() | nil) :: boolean
  def is_palindrome(head) do
    nums = to_arr(head, [])
    nums == Enum.reverse(nums)
  end

  defp to_arr(nil, nums), do: nums
  defp to_arr(node, nums), do: to_arr(node.next, [node.val | nums])
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        nums: [1, 2, 2, 1]
      },
      %{
        nums: [1, 2]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums |> Enum.reverse() |> arr_to_list(nil)
    result = Solution.is_palindrome(nums)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil

  def arr_to_list([num | nums], head) do
    arr_to_list(nums, %ListNode{val: num, next: head})
  end

  def arr_to_list([], head), do: head
end

Main.main()

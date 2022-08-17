defmodule ListNode do
  @type t :: %__MODULE__{
          val: integer,
          next: ListNode.t() | nil
        }
  defstruct val: 0, next: nil
end

defmodule Solution do
  @spec remove_elements(head :: ListNode.t() | nil, val :: integer) :: ListNode.t() | nil
  def remove_elements(nil, _), do: nil

  def remove_elements(head, val) when head.val != val,
    do: %ListNode{val: head.val, next: remove_elements(head.next, val)}

  def remove_elements(head, val), do: remove_elements(head.next, val)
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        head: arr_to_list([1, 2, 6, 3, 4, 5, 6] |> Enum.reverse(), nil),
        val: 6
      },
      %{
        head: arr_to_list([], nil),
        val: 1
      },
      %{
        head: arr_to_list([7, 7, 7, 7], nil),
        val: 7
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    val = input.val
    head = input.head
    result = Solution.remove_elements(head, val)
    IO.puts(result |> list_to_arr([]) |> Enum.join(","))
    main(remains)
  end

  def main([]), do: nil

  defp arr_to_list([num | nums], result), do: arr_to_list(nums, %ListNode{val: num, next: result})
  defp arr_to_list([], result), do: result

  @spec list_to_arr(node :: ListNode.t() | nil, nums :: [integer]) :: [integer]
  defp list_to_arr(nil, nums), do: Enum.reverse(nums)
  defp list_to_arr(node, nums), do: list_to_arr(node.next, [node.val | nums])
end

Main.main()

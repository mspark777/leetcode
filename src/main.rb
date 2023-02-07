# @param fruits [Array<Integer>]
# @return [Integer]
def total_fruit(fruits)
  # @type [Hash<Integer, Integer>]
  baskets = Hash.new(0)

  left = 0
  result = 0

  fruits.each_with_index do |rfruit, right|
    baskets[rfruit] += 1

    while baskets.length > 2
      lfruit = fruits[left]
      baskets[lfruit] -= 1
      baskets.delete(lfruit) if baskets[lfruit] == 0

      left += 1
    end

    result = [result, right - left + 1].max
  end

  result
end

def main
  inputs = [
    [1, 2, 1],
    [0, 1, 2, 2],
    [1, 2, 3, 2, 2],
    [3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]
  ]

  inputs.each do |input|
    result = total_fruit(input)
    puts(result)
  end
end

main

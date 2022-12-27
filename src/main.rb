# @param capacity [Array<Integer>]
# @param rocks [Array<Integer>]
# @param additional_rocks [Integer]
# @return [Integer]
def maximum_bags(capacity, rocks, additional_rocks)
  # @type [Array<Integer>]
  remains = capacity.map.with_index { |c, i| c - rocks[i] }
  remains.sort!

  result = 0

  for remain in remains do
    break unless additional_rocks >= remain

    additional_rocks -= remain
    result += 1
  end

  result
end

def main
  inputs = [
    {
      capacity: [2, 3, 4, 5],
      rocks: [1, 2, 4, 4],
      additional_rocks: 2
    },
    {
      capacity: [10, 2, 2],
      rocks: [2, 2, 0],
      additional_rocks: 100
    }
  ]

  inputs.each do |input|
    result = maximum_bags input[:capacity], input[:rocks], input[:additional_rocks]
    puts result
  end
end

main

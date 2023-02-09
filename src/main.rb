# @param ideas [Array<String>]
# @return [Integer]
def distinct_names(ideas)
  # @type [Hash<String, Hash<String, Boolean>>]
  group_map = Hash.new do |hash, key|
    hash[key] = {}
  end

  ideas.each do |idea|
    first = idea[0]
    remains = idea[1..]
    group_map[first][remains] = true
  end

  result = 0

  groups = group_map.values
  for i in (0..(groups.length - 2))
    cur = groups[i]
    for j in ((i + 1)..(group_map.length - 1))
      group = groups[j]
      num = 0

      cur.keys.each do |idea|
        num += 1 if group.has_key?(idea)
      end

      result += 2 * (cur.size - num) * (group.size - num)
    end
  end

  result
end

def main
  inputs = [%w[coffee donuts time toffee], %w[lack back]]

  inputs.each do |input|
    result = distinct_names(input)
    puts(result)
  end
end

main

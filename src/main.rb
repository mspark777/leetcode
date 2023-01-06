# @param costs [Array<Integer>]
# @param coins [Integer]
# @return [Integer]
def max_ice_cream(costs, coins)
  costs.sort!

  result = 0
  costs.each do |cost|
    break if coins < cost

    coins -= cost
    result += 1
  end
  result
end

def main
  inputs = [
    { costs: [1, 3, 2, 4, 1], coins: 7 },
    { costs: [10, 6, 8, 7, 7, 8], coins: 5 },
    { costs: [1, 6, 3, 1, 2, 5], coins: 20 }
  ]

  inputs.each do |input|
    result = max_ice_cream input[:costs], input[:coins]
    puts result
  end
end

main

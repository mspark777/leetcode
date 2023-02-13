# @param {Integer} low
# @param {Integer} high
# @return {Integer}
def count_odds(low, high)
  low += 1 if (low & 1) == 0

  return 0 if low > high

  ((high - low) / 2) + 1
end

def main
  inputs = [
    [3, 7],
    [8, 10]
  ]

  inputs.each do |input|
    result = count_odds(input[0], input[1])
    puts(result)
  end
end

main

# @param num [Integer]
# @return [Boolean]
def is_perfect_square(num)
  left = 1
  right = num / 2
  while left <= right
    mid = (left + right) / 2
    square = mid * mid
    if num < square
      right = mid - 1
    elsif num > square
      left = mid + 1
    else
      return true
    end
  end

  num == 1
end

def main
  inputs = [
    16, 14, 1
  ]

  inputs.each do |num|
    result = is_perfect_square num
    puts result
  end
end

main

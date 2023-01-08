# @param a [Integer]
# @param b [Integer]
# @return [Integer]
def GCD(a, b)
  return a if b == 0

  GCD(b, a % b)
end

# @param points [Array<Array<Integer>>]
# @return [Integer]
def max_points(points)
  plength = points.length
  return 1 if plength < 2

  result = 2

  points.each_index do |i|
    slopes = Hash.new(0)
    points.each_index do |j|
      next if i == j

      x = points[j][0] - points[i][0]
      y = points[j][1] - points[i][1]
      gcd = GCD x.abs, y.abs
      if gcd != 0
        x /= gcd
        y /= gcd
      end

      key = format('%d:%d', x, y)
      slopes[key] += 1
    end

    slopes.each_value do |value|
      result = [result, value + 1].max
    end
  end

  result
end

def main
  inputs = [
    [[1, 1], [2, 2], [3, 3]],
    [[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]
  ]

  inputs.each do |points|
    result = max_points points
    puts result
  end
end

main

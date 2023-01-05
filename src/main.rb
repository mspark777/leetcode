# @param points [Array<Array<Integer>>]
# @return [Integer]
def find_min_arrow_shots(points)
  points.sort_by! { |p| p[1] }

  result = 1
  prev = 0

  points[1..-1].each_with_index do |p, cur|
    if p[0] > points[prev][1]
      result += 1
      prev = cur + 1
    end
  end

  result
end

def main
  inputs = [
    [[10, 16], [2, 8], [1, 6], [7, 12]],
    [[1, 2], [3, 4], [5, 6], [7, 8]],
    [[1, 2], [2, 3], [3, 4], [4, 5]]
  ]

  inputs.each do |points|
    result = find_min_arrow_shots points
    puts result
  end
end

main

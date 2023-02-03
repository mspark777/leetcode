# @param s [String]
# @param num_rows [Integer]
# @return [String]
def convert(s, num_rows)
  return s if num_rows <= 1

  last_row = num_rows - 1
  row = 0
  down = true

  # @type [Array<Array<String>>]
  result = Array.new(num_rows) { [] }
  s.each_char do |ch|
    result[row].push(ch)
    if row == last_row
      down = false
    elsif row == 0
      down = true
    end

    if down
      row += 1
    else
      row -= 1
    end
  end

  result.map { |arr| arr.join('') }.join('')
end

def main
  inputs = [
    { s: 'PAYPALISHIRING', num_rows: 3 },
    { s: 'PAYPALISHIRING', num_rows: 4 },
    { s: 'A', num_rows: 1 },
    { s: 'AB', num_rows: 1 }
  ]

  inputs.each do |input|
    result = convert(input[:s], input[:num_rows])
    puts(result)
  end
end

main

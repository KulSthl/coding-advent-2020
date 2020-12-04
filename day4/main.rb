file = File.open('data.txt')
file_data = file.read
file_data = file_data.lines.map(&:chomp)
file_data.each do |_line|
  puts true if file_data[6] == ''
end
formatted = []
def splitter(start_index, array, formatted)
  return if start_index >= array.length

  counter = start_index
  temp_array = []
  while array[counter] != '' && counter < array.length
    temp_array += array[counter].split
    counter += 1
  end
  formatted.push(temp_array)
  splitter(counter + 1, array, formatted)
end
splitter(0, file_data, formatted)
def field_analyser(formatted)
  number_pass_part1 = 0
  number_pass_part2 = 0
  formatted.each do |lcard|
    card = lcard
    valid = true
    valid = false unless card.join.include? 'byr'
    valid = false unless card.join.include? 'iyr'
    valid = false unless card.join.include? 'eyr'
    valid = false unless card.join.include? 'hgt'
    valid = false unless card.join.include? 'hcl'
    valid = false unless card.join.include? 'ecl'
    valid = false unless card.join.include? 'pid'
    if valid == true
      number_pass_part1 += 1
      lcard.each do |field|
        pair = field.strip.split(':')
        value = pair[1]
        name = pair[0]
        if name == 'hgt'
          if value.include?('cm')
            valid = false unless value.to_i >= 150 && value.to_i <= 193
          elsif value.include?('in')
            valid = false unless value.to_i >= 59 && value.to_i <= 76
          else
            valid = false
          end
        end
        valid = false if name == 'byr' && !(value.to_i >= 1920 && value.to_i <= 2002)
        valid = false if name == 'iyr' && !(value.to_i >= 2010 && value.to_i <= 2020)
        valid = false if name == 'eyr' && !(value.to_i >= 2020 && value.to_i <= 2030)
        valid = false if name == 'hcl' && !value.match('#[0-9a-f]{6}')
        if name == 'ecl'
          case value
          when 'amb'
          when 'blu'
          when 'brn'
          when 'gry'
          when 'grn'
          when 'hzl'
          when 'oth'
          else
            valid = false
          end
        end
        valid = false if name == 'pid' && !(value.length == 9 && value.match('[0-9]{9}'))
      end
    end
    number_pass_part2 += 1 if valid == true
  end
  puts number_pass_part1
  puts number_pass_part2
end
field_analyser(formatted)

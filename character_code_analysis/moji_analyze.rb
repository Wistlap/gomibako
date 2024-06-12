require 'nkf'

file_path = './docs/1.txt'
moji_code = NKF.guess(File.read(file_path)).to_s
puts "1.txt: #{moji_code}"

file_path = './docs/2.txt'
moji_code = NKF.guess(File.read(file_path)).to_s
puts "2.txt: #{moji_code}"

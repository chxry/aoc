module Day3 where
import Data.Char
import Data.List
import Data.Maybe

maxjoltage 1 xs = maximum xs
maxjoltage n xs =
  let d = maximum (take (length xs - n + 1) xs)
  in d * 10^(n - 1) + maxjoltage (n - 1) (tail (dropWhile (/=d) xs))

sol n s =
  let banks = map (map digitToInt) (lines s)
  in sum (map (maxjoltage n) banks)

sol1 = sol 2
sol2 = sol 12

input = readFile "3.txt"
test = fmap sol2 input

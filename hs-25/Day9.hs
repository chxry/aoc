module Day9 where
import Data.List
import Data.List.Split
import Util

parseCoord s =
  let [a, b] = splitOn "," s
  in (read a, read b)

area ((a, b), (c, d)) = (abs (c - a) + 1) * (abs (d - b) + 1)

sol1 s =
  let cs = map parseCoord $ lines s
  in maximum $ map area $ pairs cs

noOverlap ((a1, b1), (c1, d1)) ((a2, b2), (c2, d2)) =
  max a2 c2 <= min a1 c1 || max a1 c1 <= min a2 c2 ||
  max b2 d2 <= min b1 d1 || max b1 d1 <= min b2 d2

sol2 s = 
  let cs = map parseCoord $ lines s
      valid r = all (noOverlap r) $ zip cs $ tail cs ++ [head cs]
  in maximum $ map area $ filter valid $ pairs cs

input = readFile "9.txt"
test = fmap sol2 input

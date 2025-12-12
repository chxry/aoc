module Day12 where
import Data.List
import Data.List.Split
import Grid

-- parseShape s = gridFromLists $ tail $ lines s
parseShape s = length $ filter (=='#') $ concat $ lines s

parseTree s =
  let (x, rest) = break (=='x') s
      (y, rest') = break (==':') $ tail rest
  in (read x, read y, map read $ words $ tail rest')

parse s =
  let bs = splitOn "\n\n" s
      shapes = map parseShape $ init bs
      trees = map parseTree $ lines $ last bs
  in (shapes, trees)

sol1 s =
  let (shapes, trees) = parse s
      -- both these criteria work ?? possibly the least satisfying ending to a puzzle
      f (x, y, ps) = x * y >= (sum $ zipWith (*) ps shapes)
      g (x, y, ps) = x / 3 * y / 3 >= sum ps
  in length $ filter g trees

input = readFile "12.txt"
test = fmap sol1 input

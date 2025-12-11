module Day11 where
import Data.List
import qualified Data.Map as M

parse s = M.fromList [(x, xs) | l <- lines s, let (x:xs) = words (delete ':' l)]

count ls start = go start
  where
    go "out" = 1
    go x = next M.! x
    next = M.fromList [(x, sum [go y | y <- ls M.! x]) | x <- M.keys ls]

count2 ls start = go start False False
  where
    go "out" d f | d && f = 1
                 | otherwise = 0
    go x d f = next M.! (x, d, f)
    next = M.fromList [((x, d, f), sum [go y (d || y=="dac") (f || y=="fft") | y <- ls M.! x]) | x <- M.keys ls, d <- [False, True], f <- [False, True] ]

sol1 s =
  let ls = parse s
  in count ls "you"

sol2 s =
  let ls = parse s
  in count2 ls "svr"

input = readFile "11.txt"
test = fmap sol2 input

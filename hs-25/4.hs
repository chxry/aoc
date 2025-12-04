import Grid

remove g@(G w h xs) = gridFromList w h [f (get g (x, y)) n | y <- [0..h - 1], x <- [0..w - 1], let n = sum (neighbours g 0 (x, y))]
  where 
      f 1 n | n < 4 = 0
      f x n = x

count = sum . getList

parse s = mapGrid (\c -> if c == '@' then 1 else 0) (readGrid s)

sol1 s =
  let grid = parse s
  in count grid - count (remove grid)

sol2 s =
  let grid = parse s
      f g = let g' = remove g
            in if g' == g then g else f g'
  in count grid - count (f grid)

input = readFile "4.txt"
test = fmap sol2 input

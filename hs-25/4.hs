neighbours (x, y) = [(x + dx, y + dy) | dx <- [-1, 0, 1], dy <- [-1, 0, 1], dx /= 0 || dy /= 0]

get g (x, y) | x >= 0 && x < (width g) && y >= 0 && y < (height g) && (g !! y) !! x == '@' = 1
             | otherwise = 0

width = length . head

height = length

remove g = [(x, y) | x <- [0..width g - 1], y <- [0..height g - 1], get g (x, y) == 1, let n = sum (map (get g) (neighbours (x, y))), n < 4]

replace i x xs = take i xs ++ x : drop (i + 1) xs

remove2 g = [[f (x, y) | x <- [0..width g - 1]] | y <- [0..height g - 1]]
  where f (x, y) | get g (x, y) == 1 && n >= 4 = '@'
                 | otherwise = '.'
          where n = sum (map (get g) (neighbours (x, y)))

sol1 s =
  let grid = lines s
  in length (remove grid)

sol2 s =
  -- let grid = lines s
  --     f g = let d = remove g
  --           in (foldr (\(x, y) g -> replace y (replace x '.' (g !! y)) g) g d, length d)
  --     h g n | g' == g = n + dn
  --           | otherwise = h g' (n + dn)
  --       where (g', dn) = f g
  -- in h grid 0
  let grid = lines s
      h g | remove2 g == g = g
          | otherwise = h (remove2 g)
      count g = sum (concatMap (map (\x -> if x == '@' then 1 else 0)) g)
  in count grid - count (h grid)
      
input = readFile "4.txt"
test = sol2 <$> input

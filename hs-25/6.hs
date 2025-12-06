import Data.List

op '+' xs = sum xs
op '*' xs = product xs

eval xs = op (head (last xs)) (map read (init xs))

sol1 s = sum (map eval (transpose (map words (lines s))))

spliti [] xs = [xs]
spliti (i:is) xs =
  let (a, b) = splitAt i xs
  in a:spliti (map (subtract i) is) b

nonempty = any (/=' ')

sol2 s =
  let ls = lines s
      ops = last ls
      ops' = filter (/=' ') ops
      offsets = tail [i | (x, i) <- zip ops [0..], x /= ' ']
      rows = map (spliti offsets) (init ls)
  in sum (zipWith op ops' (map (map read . filter nonempty .transpose) (transpose rows)))

input = readFile "6.txt"
test = fmap sol2 input

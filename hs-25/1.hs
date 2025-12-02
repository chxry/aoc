rotate 'L' 0 = 99
rotate 'L' n = n - 1
rotate 'R' 99 = 0
rotate 'R' n = n + 1

applyN n f x = iterate f x !! n
instr n (x:xs) = applyN (read xs) (rotate x) n

applyN2 n f x = take n (tail (iterate f x))
instr2 (n,k) (x:xs) =
  let as = applyN2 (read xs) (rotate x) n
  in (last as, length (filter (==0) as))

sol1 s =
  let positions = scanl instr 50 (lines s)
  in length (filter (==0) positions)

sol2 s =
  let zs = scanl instr2 (50, 0) (lines s)
  in sum (map snd zs)

input = readFile "1.txt"
test = sol2 <$> input

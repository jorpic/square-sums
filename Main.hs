
import qualified Data.IntMap as Map
import qualified Data.IntSet as Set
import Data.IntMap ((!))
import Data.IntSet ((\\))
import Data.List (nub, sort, nubBy)
import Control.Monad (forM_, guard)

data Res = Res [Int] Set.IntSet

_N = 1000 :: Int
squares = Set.fromAscList [x*x | x <- [2..2*_N]]
isSquare x = Set.member x squares

type Solution = [Int]

solveFrom :: (Int, Int) -> Int -> [Solution]
solveFrom (a,b) x = do
  sol <- loop (a, b) (Set.singleton x)  x
  guard (length sol == b-a)
  return $ sol

solve :: (Int, Int) -> [Solution]
solve (a,b) = [x:xs | x <- [a..b], xs <- solveFrom (a,b) x]

loop :: (Int, Int) -> Set.IntSet -> Int -> [Solution]
loop (a, b) used x
  | Set.size used == b - a + 1 = [[]]
  | otherwise = do
    let siblings =
          [ y
          | y <- [a..b]
          , y /= x
          , not $ Set.member y used
          , isSquare (x+y)]
    y <- siblings
    xs <- loop (a, b) (Set.insert y $ Set.filter (>=a) used) y
    return $ y:xs

check :: (Int, Int) -> Solution -> Bool
check (a, b) xs
  =  sort xs == [a..b]
  && all isSquare (zipWith (+) xs (tail xs))


hSolve = step1 ++ step2
  where
    step1 =
      [ ([(i, last s)], s)
      | i <- [15..40]
      , s <- nubBy (\a b -> last a == last b) $ solve (1,i)
      ]
    step2 =
      [ ((j+k, last s2) : prev, s1 ++ s2)
      | (prev@((j, x):_), s1) <- step1
      , k <- [1..63]
      , s2 <- solveFrom (j, j+k) x
      ]
    step3 =
      [ ((j+k, last s2) : prev, s1 ++ s2)
      | (prev@((j, x):_), s1) <- step2
      , k <- [1..133]
      , s2 <- solveFrom (j, j+k) x
      ]


main = do
  forM_ [15..35] $ \n -> forM_ (solve (1,n)) $ \s -> print (n, s)
  -- forM_ hSolve $ \(res@((n, _):_), sol) -> print (res, check (1, n) sol)

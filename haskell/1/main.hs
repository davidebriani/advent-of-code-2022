import Data.List
import Data.List.Split
import System.Environment (getArgs)

type Calories = Int
type ElfCalories = [Calories]

main :: IO ()
main = do
  args <- getArgs
  case args of
    ["part1"] -> part1
    ["part2"] -> part2
    _         -> usage

part1 :: IO ()
part1 = interact $ show . solve1 . parseCalories . splitWhen (== "") . lines

part2 :: IO ()
part2 = interact $ show . solve2 . parseCalories . splitWhen (== "") . lines

usage :: IO ()
usage = do
  putStrLn "Wrong parameters. Usage:"
  putStrLn "$ main <part1|part2>"

solve1 :: [ElfCalories] -> Calories
solve1 = maximum . fmap sum

solve2 :: [ElfCalories] -> Calories
solve2 = sum . take 3 . reverse . sort . fmap sum

parseCalories :: [[String]] -> [ElfCalories]
parseCalories = fmap (fmap read)

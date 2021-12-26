import System.Environment  


readInts :: String -> [Int]
readInts s = map (read::String->Int) $ words s

minMax :: Ord a => [a] -> Maybe (a, a)
minMax (h: t) = Just $ foldr (\x (sm, lg) -> (min sm x, max lg x)) (h, h) t
minMax _ = Nothing

main :: IO ()
main = do
    contents <- readFile "programming/haskell/files/numbers.txt"
    let values = readInts contents
        count = length values
        total = sum values
        mean = fromIntegral total / fromIntegral count
        range = minMax values
    print values
    print count
    print total
    print mean
    print range

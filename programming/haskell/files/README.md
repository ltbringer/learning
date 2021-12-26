# Files

A program that takes in a file containing numbers and produces a bunch of statistics about the file.

```txt
25 5 39 23 8
60 71 79 90 82
96 69 53 62 73
59 86 33 18 51
83 52 86 84 62
27 76 58 32 61
23 43 70 56 80
25 10 30 29 7
57 74 31 90 87
19 45 13 81 90
```

We want to write the following functions:

1. Read the file.
2. Log the following on stdout.
    1. The values in the file.
    2. The total number of values in the file.
    3. The sum of all the numbers.
    4. The mean of all the values.
    5. The smallest and largest value in the file. The range of the numbers.

## minMax

```haskell
minMax :: Ord a => [a] -> Maybe (a, a)
minMax (h: t) = Just $ foldr (\x (sm, lg) -> (min sm x, max lg x)) (h, h) t
minMax _ = Nothing
```
I found the above a little hard to approach to so I'll break it down for my future self.

- `(\x (sm, lg) -> (min sm x, max lg x))` apply this function by taking `(h, h)` as the initial values and apply on `t` i.e. rest of the elements.
- We check if the current item `h` is the `min` or `max` and the result is available in the next iteration as `(sm, lg)` i.e. `if h < t[1]` then `sm` would be `h` else `sm` would be `t[1]`. Do note; `t[1]` isn't available, we are just using the notation to understand the program.
- So we go through every element in the list and check if the previous `sm` is updated with the smallest value in the list.
- The same applies to `lg`.

import Test.Tasty
import Test.Tasty.HUnit

import Lib

main :: IO ()
main = do
  defaultMain (testGroup "Our Library Tests" [robotCombat])

robotCombat :: TestTree
robotCombat = testCase "Testing combat" $ do
    let giantBot = makeRobot ("Huge-o", 10, 10000)
    let strongBot = makeRobot ("Strongman", 100, 100)
    let glassCannon = makeRobot ("Glass Cannon", 1000, 1)
    let robots = [giantBot, strongBot, glassCannon]

  (assertEqual "Should say Yo to Friend!" "Yo Friend!" (sayYo "Friend"))

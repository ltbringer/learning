import Test.Tasty
import Test.Tasty.HUnit

import Lib


main :: IO ()
main = do
  defaultMain (testGroup "Our Library Tests" [testACupOf50ml, testASipFromCup, testCupIsEmpty, testCupConsumption])

testACupOf50ml :: TestTree
testACupOf50ml = testCase "Measure a cup for 50ml coffee" $ do
    let qty = 50
        a50mlCup = cup qty
        message = "Should be equal to " ++ (show qty)
    (assertEqual message (getQty a50mlCup) qty)

testASipFromCup :: TestTree
testASipFromCup = testCase "Sip an ml from a cup" $ do
    let qty = 50
        a50mlCup = cup qty
        sipQty = 1
        sipCup = drink a50mlCup sipQty
        message = "Should be equal to " ++ (show (qty - sipQty))
    (assertEqual message (getQty sipCup) (qty - sipQty))

testCupIsEmpty :: TestTree
testCupIsEmpty = testCase "A cup is empty" $ do
    let emptyCup = cup 0
    (assertEqual "Should be empty" (isEmpty emptyCup) True)

testCupConsumption :: TestTree
testCupConsumption = testCase "A cup is consumed" $ do
    let qty = 50
        a50mlCup = cup qty
        consumers = [1, 4, 5, 6]
        sipCup = consume a50mlCup consumers
    (assertEqual "Should have 34 ml" (getQty sipCup) 34)

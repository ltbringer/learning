import Test.Hspec
import Data
import Lib

main :: IO ()
main = hspec $ do
    describe "formatGrid" $ do
        it "Should concatenate everyline with a \\n." $ do
            (formatGrid ["abc", "def", "ghi"]) `shouldBe` "abc\ndef\nghi"

    describe "findWord" $ do
        it "Should return the word if it is in the grid." $ do
            (findWord grid "hello") `shouldBe` Nothing
            (findWord grid "HASKELL") `shouldBe` Just "HASKELL"

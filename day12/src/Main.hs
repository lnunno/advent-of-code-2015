{-# LANGUAGE OverloadedStrings #-}
module Main where

import Data.Aeson
import Data.Scientific
import qualified Data.Map as M
import qualified Data.Vector as V
import qualified Data.ByteString.Lazy as B
import qualified Data.HashMap.Lazy as H
import qualified Data.Text as T

foo :: H.HashMap String String
foo = H.singleton "foo" "bar"

sumNumbers :: Value -> Scientific
sumNumbers (Number a) = a
sumNumbers (Array a) = V.sum $ V.map sumNumbers a
sumNumbers (String s) = 0
sumNumbers (Bool b) = 0
sumNumbers (Object o) = 0

jsonFile :: FilePath
jsonFile = "test-input.json"

getJSON :: IO B.ByteString
getJSON = B.readFile jsonFile

retrieveJSON :: IO Value
retrieveJSON = do
    byteString <- getJSON
    let d = decode byteString :: Maybe Value
    case d of
        Just json -> return json
        Nothing -> error "Could not parse JSON file."

main :: IO ()
main = do
    json <- retrieveJSON
    print json
    print $ sumNumbers json

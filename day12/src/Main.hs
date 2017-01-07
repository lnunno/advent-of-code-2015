{-# LANGUAGE OverloadedStrings #-}
module Main where

import Data.Aeson
import qualified Data.ByteString.Lazy as B

jsonFile :: FilePath
jsonFile = "input.json"

getJSON :: IO B.ByteString
getJSON = B.readFile jsonFile

main :: IO ()
main = do
    byteString <- getJSON
    let d = decode byteString :: Maybe Object
    case d of
        Just obj -> print obj
        Nothing -> error "Could not parse JSON file."

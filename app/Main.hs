{-# LANGUAGE ForeignFunctionInterface #-}

import Foreign (Ptr, castPtr)
import Foreign.C.Types (CInt(..))
import Foreign.Marshal.Array (withArrayLen, peekArray)

foreign import ccall "quicksort"
  c_quicksort :: Ptr CInt -> CInt -> IO ()

rustQuicksort :: [Int] -> IO [Int]
rustQuicksort hsList = do
  withArrayLen (map fromIntegral hsList) $ \len arr -> do
    c_quicksort arr (fromIntegral len)
    peekArray len (castPtr arr) >>= return . map (fromIntegral :: CInt -> Int)

main :: IO ()
main = do
  let unsorted = [3, 6, 1, 9, 21, 10, 4]

  sorted <- rustQuicksort unsorted

  putStrLn $ "Unsorted: " ++ show unsorted
  putStrLn $ "sorted: " ++ show sorted


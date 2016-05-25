module Data.Tes3.Parser.Native where

#include <haskell>
import Data.Tes3
import Data.Tes3.Utils

pT3FileSignature :: T.Parser ()
pT3FileSignature = do
  void $ Tp.string "3SET"
  Tp.endOfLine

t3FileRef :: T.Parser T3FileRef
t3FileRef = do
  n <- pNulledRun
  void $ Tp.char ' '
  z <- Tp.decimal
  Tp.endOfLine
  return $ T3FileRef n z

pT3FileHeader :: T.Parser T3FileHeader
pT3FileHeader = do
  void $ Tp.string "VERSION "
  version <- Tp.decimal
  Tp.endOfLine
  void $ Tp.string "TYPE "
  file_type <- pT3FileType
  Tp.endOfLine
  void $ Tp.string "AUTHOR "
  author <- pLine
  void $ Tp.string "DESCRIPTION"
  Tp.endOfLine
  description <- pLines
  refs <- many t3FileRef
  return $ T3FileHeader version file_type author description refs

pT3Record :: T.Parser T3Record
pT3Record = do
  s <- pT3Sign
  g <- Tp.option 0 $ Tp.char ' ' >> Tp.decimal
  Tp.endOfLine
  fields <- many $ t3Field s
  return $ T3Record s g fields

t3Field :: T3Sign -> T.Parser T3Field
t3Field record_sign = do
  s <- pT3Sign
  t3FieldBody (t3FieldType record_sign s) s

t3FieldBody :: T3FieldType -> T3Sign -> T.Parser T3Field
t3FieldBody T3Binary s = do
  void $ Tp.char ' '
  b <- decode <$> C.pack <$> ST.unpack <$> Tp.takeTill Tp.isEndOfLine
  Tp.endOfLine
  case b of
    Left e -> fail e
    Right r -> return $ T3BinaryField s r
t3FieldBody T3String s = do
  void $ Tp.char ' '
  t <- pNulledLine
  return $ T3StringField s t
t3FieldBody T3Multiline s = do
  Tp.endOfLine
  t <- pLines
  return $ T3MultilineField s t
t3FieldBody T3MultiString s = do
  Tp.endOfLine
  t <- pLines
  return $ T3MultiStringField s t
t3FieldBody T3Ref s = do
  void $ Tp.char ' '
  n <- Tp.decimal
  void $ Tp.char ' '
  t <- pLine
  return $ T3RefField s n t
t3FieldBody (T3FixedString _) s = do
  void $ Tp.char ' '
  t <- pNulledLine
  return $ T3FixedStringField s t

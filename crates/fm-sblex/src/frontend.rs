// {-# OPTIONS_GHC -XFlexibleInstances  -XTypeSynonymInstances #-}

// module Frontend where

// import qualified Data.Set as Set
// import qualified Data.Map as Map
// import qualified Dict.Abs as Abs
// import Dictionary
// import Dict.ErrM
// import Util
// import Data.Char
// import Distribution.System
// import Print
// import General
// import Control.Monad(when)
// import Data.Maybe(isJust)
// import System.IO.Unsafe (unsafePerformIO)
// // import System.IO
// import Tokenize
// import qualified CTrie
// import EditDistance
// import Data.List
// import Compound

// -- Note that all Functions have default definitions, but
// -- in the common case, you give, at least, definitions for "paradigms"
// -- "internDict" and "composition"

// type Label = String

// type TestInput = (String, Dictionary_Word, Category, Paradigm, [String], [Inherent],String)

// tword :: TestInput -> Maybe String
// tword ("",_,_,_,_,_,_) = Nothing
// tword (s,_,_,_,_,_,_) = Just s

// w :: TestInput -> String
// w t = case (tword t) of
//            Just s -> s
//            Nothing -> []

// thead :: TestInput -> String
// thead (_,s,_,_,_,_,_) = s

// tcat :: TestInput -> String
// tcat  (_,_,s,_,_,_,_) = s

// tpara :: TestInput -> String
// tpara (_,_,_,s,_,_,_) = s

// tparam :: TestInput -> [String]
// tparam (_,_,_,_,xs,_,_) = xs

// tinhs :: TestInput -> [String]
// tinhs (_,_,_,_,_,xs,_) = xs

// tid :: TestInput -> String
// tid (_,_,_,_,_,_,s) = s

// type PositiveTests = [(TestInput -> Maybe String)]

// type NegativeTests = [(TestInput -> Maybe String)]

// type Result = Maybe String

// message :: TestInput -> String -> Result
// message t s = Just $ concat ["[ ", s, " ]\n    ", prw (w t), " {h:\"", pr (thead t),"\" pos:",pr (tcat t)," param:", prl (tparam t),
//                              " is:",pri (tinhs t), " id:", pr (tid t), " p:", pr (tpara t),"}"]
//  where prw s = case s of
//                  [] -> " * "
//                  x  -> quote s
//        pr s = case s of
//                [] -> "unknown"
//                _  -> s
//        pri xs = case xs of
//                  [] -> "none"
//                  _  -> unwords xs
//        prl xs = case xs of
//                  [] -> "unknown"
//                  _  -> unwords xs

// pass :: Maybe String
// pass = Nothing

// type Encoding = String

// type TrPos   = String -> String

// type TrInhs  = [String] -> [String]

// type TrParam = (String,String,[String]) -> String

use std::fmt::Display;

// --class Show a => Language a b | a -> b where
// -- | A class defined to be able to construct a language independent frontend
pub trait Language: Display {
    // class Show a => Language a  where
    //   name              :: a -> String
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    //   morphology_header :: a -> String
    fn morphology_header(&self) -> &'static str {
        self.name()
    }
    //   dbaseName         :: a -> String
    //   composition       :: a -> Maybe CompDesc --([General.Attr] -> Bool)
    //   word_attr         :: a -> [Attr]
    //   affixes           :: a -> Set.Set String
    //   env               :: a -> String
    //   paradigms         :: a -> Commands
    //   internDict        :: a -> Dictionary
    //   tokenizer         :: a -> String -> [General.Tok]
    //   wordGuesser       :: a -> String -> [String]
    //   termParser        :: a -> [Abs.Term] -> Entry -> Entry
    //   testBench         :: a -> (PositiveTests,NegativeTests)
    //   dup_id_exceptions :: a -> [(String,String)]
    //   sandhi_rules      :: a -> (String,String) -> [(String,String)]
    //   encoding    :: a -> Map.Map Encoding (Maybe TrPos, Maybe TrInhs, Maybe TrParam)
    //   lprinter :: a -> [(String,Dictionary -> String)]
    //   dictionary_postprocessing :: a -> Dictionary -> Dictionary
    //   paradigm_dup_exception :: a -> Set.Set String
    //   name        l = map toLower (show l)
    //   morphology_header l = name l
    //   dbaseName   l = name l ++ ".dict"
    //   composition l = Nothing -- noComp
    //   word_attr l = [0]
    //   affixes     l = Set.empty
    //   env         l = "FM_" ++ map toUpper (show l)
    //   encoding    _ = Map.empty
    //   paradigms   _ = emptyC
    //   internDict  _ = emptyDict
    //   tokenizer   _ = tokens
    //   sandhi_rules _ = (:[])
    //   wordGuesser _ = const []
    //   testBench   _ = ([],[])
    //   dup_id_exceptions _ = []
    //   termParser  _ _ e = e
    //   lprinter _ = []
    //   dictionary_postprocessing _ d = d
    //   paradigm_dup_exception _ = Set.empty
}
// -- | type for Command Map
// type Commands = Map.Map String ([String], [String] -> Entry)

// -- | empty Command Map
// emptyC :: Commands
// emptyC = Map.empty

// isComp :: Language l => l -> Bool
// isComp l = case composition l of
//              Nothing -> False
//              _       -> True

// -- | add a command
// insertCommand :: (String,[String],[String] -> Entry) -> Commands -> Commands
// insertCommand (n,args,f) cs
//   | Map.member n cs =  unsafePerformIO $
//                         do prErr $ "internal error:\nduplicated paradigm identifier in command table: " ++ n
//                            return cs
//   | otherwise       =  Map.insert n (args,f) cs

// -- | Construct a Command Map
// mkCommands :: [(String,[String],[String] -> Entry)] -> Commands
// mkCommands = foldr insertCommand Map.empty

// -- | Create a dictionary from the list of paradigms.
// -- prParadigms :: Language a => a -> Dictionary -> String
// -- prParadigms l d = prDictionary d

// --unlines $ filter (not . null) $ map pr [(unwords (p:(map quote xs)), dictionary [f xs]) | (p,(xs,f)) <- Map.toList (paradigms l)]
// -- where pr (s,d)
// --         | is_empty_dictionary d = ""
// --         | otherwise             = "{\n" ++ s ++ "\n\n" ++ prDictionary d ++ "}\n"

// prTagset :: Language a => a -> Dictionary -> String
// prTagset l d = collect_and_print (Set.empty,Set.empty,Set.empty) (unDict d)
//  where collect_and_print :: (Set.Set String, Set.Set String, Set.Set String) -> [Entry] -> String
//        collect_and_print (s,i,p) [] = concat $ ["{\n\"pos\":[",
//                                                 (concat (intersperse "," (map quote (Set.toList s)))),
//                                                 "],\n",
//                                                 "\"inherent\":[",
//                                                 (concat (intersperse "," (map quote (Set.toList i)))),
//                                                 "],\n",
//                                                 "\"param\":[",
//                                                 (concat (intersperse "," (map quote (Set.toList p)))),
//                                                 "]\n}\n"]
//        collect_and_print (s,i,p) ((_,_, _ , pos, inhs, infl,_):xs) = collect_and_print (update [pos] s, update inhs i, update [t | (t,_) <- infl] p) xs
//        update     [] s = s
//        update (x:xs) s = update xs (Set.insert x s)

// prCompound :: Language a => a -> String
// prCompound l = case (composition l) of
//                  Nothing -> "NONE"
//                  Just x  -> prCompDesc x

// prExtract :: Language a => a -> String
// prExtract l = concat [pr p (entrywords (f xs)) | (p,(xs,f)) <- Map.toList (paradigms l)]
//  where
//   pr p ([],[]) = []
//   pr p (s,xs)
//      -- do not include multi-word paradigms or numbers.
//    | contains_space (s:xs) || contains_digits (s:xs) = []
//    | otherwise             = print_paradigm p (s:xs) (commonSubsequences (s:xs))
//   contains_space xs = or [ elem ' ' x  | x <- xs]
//   contains_digits xs = or [ any isDigit x  | x <- xs]

// print_paradigm :: String -> [String] -> [String] -> String
// print_paradigm name xs@(x:_) cs =
//  let vars = concat (intersperse "," (zipWith (++) cs (repeat ":char+"))) in
//   unlines
//    ["paradigm " ++ name ++
//     if null vars then "" else " [" ++ vars ++ "]",
//    " = " ++ (print (transform cs [] x)),
//    " {", (splitLines (intersperse "|" (map (print . (transform cs [])) (nub xs)))), " };\n"
//   ]
//  where print [] = []
//        print ((s,b):xs)
//         | b         = concat [s,if_conc xs,print xs]
//         | otherwise = concat ["\"",s,"\"",if_conc xs,print xs]
//        if_conc [] = []
//        if_conc _  = "+"
//        splitLines [] = []
//        splitLines xs = case splitAt 6 xs of
//                         (ys,[]) -> (" " ++ unwords ys)
//                         (ys,zs) -> (" " ++ unwords ys ++ "\n") ++ splitLines zs
//        transform _    []  [] = []
//        transform _ (x:xs)  [] = [(reverse (x:xs),False)]
//        transform cs ws  (x:xs) = case [z | z <- reverse (inits (x:xs)), elem z cs] of
// 			       (y:_) | null ws -> (y,True):
// 					       transform (delete y cs) [] (drop (length y) (x:xs))
// 			       (y:_) -> (reverse ws,False):(y,True):
// 			                       transform (delete y cs) [] (drop (length y) (x:xs))
// 			       _     -> transform cs (x:ws) xs

// lookup_paradigm :: Language l => l -> IO()
// lookup_paradigm l = do s <- getContents
//                        let ls = head $ filter (not.null) (lines s)
//                        putStrLn $ lparadigm $ (commas ls)
//    where
//          trim s = unwords $ words s
//          commas :: String -> [String]
//          commas [] = []
//          commas s = case span (/=',') s of
//                     (x,[]) -> [trim x]
//                     (x,(_:rest)) -> trim x:commas rest
//          lparadigm :: [String] -> String
//          lparadigm (x:xs) =
//              case span (/=':') x of
//                (x,[]) -> "[]"
//                (x,(_:pos)) ->
//                    let ps = [p | (p,e) <- [(p,f [x]) | (p,(_,f)) <- Map.toList (paradigms l)], get_pos e == pos,all (\w -> elem w (snd (entrywords e))) xs] in
//                     concat $ "[":(intersperse ",\n" ["\"" ++ p ++ "\"" | p <- ps])++["]"]
//          para = paradigms l

// printErrors :: ParadigmErrors -> (Bool, Bool) -> IO()
// printErrors _ (False,False) = return ()
// printErrors (unknowns, wrong_arguments) (ub,wb) =
//     prStd (concat
//            [if ub then printUnknowns (Set.toList unknowns) else "",
//             if wb then printWrongArguments (Set.toList wrong_arguments) else ""])
//  where printUnknowns [] =  "No undefined paradigms detected!\n\n"
//        printUnknowns xs =
//            "Undefined paradigms detected\n\n" ++ (prErrorTable xs) ++ "\n"
//        printWrongArguments [] = "No argument count mismatches detected!\n"
//        printWrongArguments xs =
//            "Argument count mismatches detected\n\n" ++
//                       (prErrorTable (map fst xs))

// check_lemma_duplication :: Dictionary -> IO()
// check_lemma_duplication d = prStd $
//                             case duplicated_lemma_id d of
//                              [] -> "No lemma id duplications detected!\n"
//                              xs -> "Lemma id duplications detected\n\n" ++
//                                    (prErrorTable xs)

// -- | List paradigm names
// paradigmNames :: Language a => a -> [String]
// paradigmNames l = [ c ++ " \"" ++ unwords args ++ "\" ;" | (c,(args,_)) <- Map.toList (paradigms l)]

// paradigmID ::  Language a => a -> [String]
// paradigmID l = [ c | (c,_) <- Map.toList (paradigms l)]

// -- | Number of paradigms.
// paradigmCount :: Language a => a -> Int
// paradigmCount l = length $ Map.toList (paradigms l)

// -- | Is input string a paradigm identifier?
// isParadigm :: Language a => a -> String -> Bool
// isParadigm l s = isJust $ Map.lookup s (paradigms l)

// class App a where
//     app :: a -> [String] -> Entry
//     arity :: a -> Int

// instance App Entry where
//     app e [] = e
//     app e xs = error $ "Too many arguments, got " ++ show (length xs) ++ " wanted 0"
//     arity _  = 0

// instance App a => App (String -> a) where
//  app f ys@(x:xs) | length ys == arity f = app (f x) (xs)
//                  | otherwise = error $ "Wrong number of arguments, got " ++ show (length ys) ++ " wanted " ++ show (arity f) ++ " in arguments: '" ++ (unwords ys) ++ "'"
//  arity f = 1 + arity (f undefined)

// -- paradigm ::  String -> (String, [String], [String] -> Entry)
// paradigm :: (App a) => String -> [String] -> a -> (String, [String], [String] -> Dictionary.Entry)
// paradigm id exs f = (id, exs, set_paradigm_id id . app f)

// paradigm_h :: (App a) => String -> [String] -> a -> (String, [String], [String] -> Dictionary.Entry)
// paradigm_h id exs f = (id, exs, \xs -> case xs of
//                                          []     -> set_paradigm_id id $ app f xs
//                                          (x:xs) -> set_head x $ set_paradigm_id id $ app f (x:xs))

// -- paradigm_id :: (App a) => String -> [String] -> String -> a -> (String, [String], [String] -> Dictionary.Entry)
// -- paradigm_id id exs p f = (id, exs, set_paradigm_id p . (app f))

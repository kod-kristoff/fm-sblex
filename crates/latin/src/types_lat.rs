// {-
//     Functional Morphology: Latin type system
//     Copyright (C) 2004  Author: Markus Forsberg

//     This program is free software; you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation; either version 2 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.

//     You should have received a copy of the GNU General Public License
//     along with this program; if not, write to the Free Software
//     Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
// -}
use fm_core::general::Param;

// module TypesLat where

// import General
// import Invariant

// {- Latin noun -}

// {- Latin noun inflectional parameters -}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Case {
    Nominative,
    Vocative,
    Accusative,
    Genitive,
    Dative,
    Ablative,
}

impl Param for Case {} // where values = enum

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Number {
    Singular,
    Plural,
}
//  deriving (Show,Eq,Enum,Ord,Bounded)

impl Param for Number {} // where values = enum

/// Latin noun inherent parameter

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}
//  deriving (Show,Eq,Enum,Ord,Bounded)

impl Param for Gender {} // where values = enum

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct NounForm(Number, Case);
// pub struct NounForm (Number,Case);
//  deriving (Show,Eq,Ord)

impl Param for NounForm {} // where

//     values = [NounForm n c | n <- values , c <- values]
//     prValue (NounForm n c) = unwords $ [prValue n, prValue c]

// type Noun = NounForm -> Str

// {- Latin adjectives -}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Grade {
    Positive,
    Comparative,
    Superlative,
} //  deriving (Show,Eq,Enum,Ord,Bounded)

impl Param for Grade {} // where values = enum

// pub struct AdjectiveForm Grade Gender (Number,Case);
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct AdjectiveForm(Grade, Gender, Number, Case);
//  deriving (Show,Eq)

impl Param for AdjectiveForm {} // where
                                //     values = [AdjectiveForm gr g n c |
                                // 	      gr <- values,
                                // 	      g  <- values,
                                // 	      n <- values,
                                // 	      c <- values]
                                //     prValue (AdjectiveForm gr g n c) =
                                // 	unwords $ [prValue gr, prValue g, prValue n, prValue c]

// type Adjective = AdjectiveForm -> Str

// {- Adverbs -}

pub struct Adverb(Form, Grade);
//  deriving (Show,Eq)

impl Param for AdverbForm {} // where
                             //     values  = [AdverbForm g | g <- values]
                             //     prValue (AdverbForm g) = prValue g

// type Adverb = AdverbForm -> Str

// {- Particles  -}

pub struct ParticleForm(Invariant);
//  deriving (Show,Eq)

impl Param for ParticleForm {} // where
                               //     values     = [ParticleForm p | p <- values]
                               //     prValue _  = "Invariant"

// type Particle    = ParticleForm  -> Str

// {- Preposition -}

pub struct PrepForm(Invariant);
//  deriving (Show,Eq)

impl Param for PrepForm {} // where
                           //     values     = [PrepForm p | p <- values]
                           //     prValue _  = "Invariant"

// type Preposition = PrepForm -> Str

// {- Verb -}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Person {
    First,
    Second,
    Third,
} //  deriving (Show,Eq,Enum,Ord,Bounded)

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum PersonI {
    SecondI,
    ThirdI,
}
//  deriving (Show,Eq,Enum,Ord,Bounded)

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
    FuturePerfect,
    PluPerfect,
} //  deriving (Show,Eq,Enum,Ord,Bounded)

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum TenseI {
    PresentI,
    PerfectI,
    FutureI,
}
//  deriving (Show,Eq,Enum,Ord,Bounded)

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum TenseS {
    PresentS,
    ImperfectS,
    PerfectS,
    PluPerfectS,
} //   deriving (Show,Eq,Enum,Ord,Bounded)

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Voice {
    Active,
    Passive,
}
//  deriving (Show,Eq,Enum,Ord,Bounded)

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum VerbForm {
    Indicative(Person, Number, Tense, Voice),
    Infinitive(TenseI, Voice),
    ParticiplesFuture(Voice),
    ParticiplesPresent,
    ParticiplesPerfect,
    Subjunctive(Person, Number, TenseS, Voice),
    ImperativePresent(Number, Voice),
    ImperativeFutureActive(Number, PersonI),
    ImperativeFuturePassiveSing(PersonI),
    ImperativeFuturePassivePl,
    GerundGenitive,
    GerundDative,
    GerundAcc,
    GerundAbl,
    SupineAcc,
    SupineAblative,
} //  deriving (Show,Eq,Ord)

// {- Instance of Param -}

impl Param for Person {} // where values = enum
impl Param for PersonI {} // where values = enum
impl Param for Tense {} // where values = enum
impl Param for TenseI {} // where values = enum
impl Param for TenseS {} // where values = enum
impl Param for Voice {} // where values = enum

impl Param for VerbForm {} // where
                           //     values =
                           //      [Indicative p n t v |
                           //       v <- values,
                           //       t <- values,
                           //       n <- values,
                           //       p <- values
                           //      ] ++
                           //      [Infinitive t v | t <- values, v <- values] ++
                           //      [ParticiplesFuture v | v <- values] ++
                           //      [ParticiplesPresent,
                           //       ParticiplesPerfect] ++
                           //      [Subjunctive p n t v |
                           //       v <- values,
                           //       t <- values,
                           //       n <- values,
                           //       p <- values
                           //      ] ++
                           //      [ImperativePresent n v |
                           //       n <- values,
                           //       v <- values] ++
                           //      [ImperativeFutureActive  n p | n <- values, p <- values] ++
                           //      [ImperativeFuturePassiveSing p | p <- values] ++
                           //      [
                           //       ImperativeFuturePassivePl,
                           //       GerundGenitive, GerundDative, GerundAcc,
                           //       GerundAbl, SupineAcc, SupineAblative
                           //      ]

// type Verb = VerbForm -> Str

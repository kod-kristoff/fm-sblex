use std::{ffi::OsString, path::PathBuf, str::RMatchIndices};

use crate::frontend::Language;

use super::{command::Flag, Error};

pub fn retrieve(l: &dyn Language, xs: Vec<OsString>) -> Result<(Vec<Flag>, Vec<PathBuf>), Error> {
    let cmd = options(l);
    let matches = cmd.get_matches_from(xs);
    let dicts: Vec<PathBuf> = matches
        .get_many::<PathBuf>("dictionary_file(s)")
        .unwrap_or_default()
        .map(|p| p.to_owned())
        .collect();
    println!("{:?}", dicts);
    if matches.get_flag("inflection") {
        return Ok((vec![Flag::Infl], dicts));
    }
    todo!()
}

pub fn options(l: &dyn Language) -> clap::Command {
    use clap::{value_parser, Arg, ArgAction, Command};

    Command::new(l.name())
        .version(l.version())
        .about(l.morphology_header())
        .arg(Arg::new("inflection").short('i').long("inflection").action(ArgAction::SetTrue).help("run inflection engine (default)"))
        .arg(Arg::new("printer").short('p').long("printer").help("print using PRINTER (core, paradigms, paradigms_compact, paradigms_latex, compound, tagset, words, lex, tables, extract, gf, xml, sfst, sfstlex, sfstheader, lexc, xfst, sql, hundict, hunaffix, lmf,rdf)"))
        .arg(Arg::new("encoding").short('e').long("encoding").help("select another morphosyntactic encoding ()"))
        .arg(Arg::new("quality").short('q').long("quality").help("run tests (all, test, dup, undef, pop, argc, para)"))
        .arg(Arg::new("find").short('f').long("find").action(ArgAction::SetTrue).help("find all paradigms that predict the given word forms"))
        .arg(Arg::new("dictionary_file(s)").action(ArgAction::Append).value_parser(
            value_parser!(PathBuf)
        ))
}

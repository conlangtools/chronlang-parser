use chumsky::{prelude::*, text::{ident, whitespace}};
use crate::parser::common::*;
use crate::ast::{
  Stmt,
  TraitMember,
};

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
  let start = just("trait")
    .padded();

  let sequence = || word_chars()
    .or_not()
    .map(|opt| match opt {
      Some(ch) => ch,
      None => "".to_string(),
    });

  let notation = just("=")
    .padded()
    .ignore_then(sequence())
    .then_ignore(just("_"))
    .then(sequence())
    .map(|(before, after)| before + "_" + &after)
    .or_not();

  let member = just("default")
    .padded()
    .or_not()
    .map(|d| d.is_some())
    .then(
      ident()
      .separated_by(just("|").padded())
    )
    .then(notation)
    .map(|((default, labels), notation)| TraitMember { default, labels, notation });

  let body = member
    .separated_by(just(",").padded())
    .allow_trailing()
    .at_least(1)
    .then_ignore(whitespace())
    .delimited_by(just("{").padded(), just("}"));

  start
    .ignore_then(ident())
    .then(body)
    .map(|(label, members)| Stmt::Trait { label, members })
}

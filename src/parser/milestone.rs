use chumsky::{prelude::*, text::ident};

use crate::ast::{Spanned, Stmt, Time};

use super::common::integer;

fn time() -> impl Parser<char, Spanned<Time>, Error = Simple<char>> {
    integer()
        .then(just("-").padded().ignore_then(integer()).or_not())
        .map_with_span(|(from, to), span| {
            (
                span,
                match to {
                    Some(to) => Time::Range(from, to),
                    None => Time::Instant(from),
                },
            )
        })
}

fn language() -> impl Parser<char, Spanned<String>, Error = Simple<char>> {
    ident().map_with_span(|lang, span| (span, lang))
}

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    let start = just("@").padded();

    let time_only = start.ignore_then(time()).map(|t| Stmt::Milestone {
        time: Some(t),
        language: None,
    });

    let language_only = start.ignore_then(language()).map(|l| Stmt::Milestone {
        time: None,
        language: Some(l),
    });

    let both = start
        .ignore_then(time())
        .then_ignore(just(",").padded())
        .then(language())
        .map(|(t, l)| Stmt::Milestone {
            time: Some(t),
            language: Some(l),
        });

    choice([both.boxed(), time_only.boxed(), language_only.boxed()])
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::ast::{Stmt, Time};

    #[test]
    fn it_parses_an_instant_milestone() {
        let src = "@ 42";
        let res = parser().parse(src.to_string());
        assert_eq!(
            res,
            Ok(Stmt::Milestone {
                time: Some((2..4, Time::Instant(42))),
                language: None
            })
        )
    }

    #[test]
    fn it_parses_a_range_milestone() {
        let src = "@ 0-100";
        let res = parser().parse(src.to_string());
        assert_eq!(
            res,
            Ok(Stmt::Milestone {
                time: Some((2..7, Time::Range(0, 100))),
                language: None
            })
        )
    }

    #[test]
    fn it_parses_a_milestone_with_a_language() {
        let src = "@ 42, TokiPona";
        let res = parser().parse(src.to_string());
        assert_eq!(
            res,
            Ok(Stmt::Milestone {
                time: Some((2..4, Time::Instant(42))),
                language: Some((6..14, "TokiPona".into())),
            })
        )
    }

    #[test]
    fn it_parses_a_milestone_with_only_a_language() {
        let src = "@ TokiPona";
        let res = parser().parse(src.to_string());
        assert_eq!(
            res,
            Ok(Stmt::Milestone {
                time: None,
                language: Some((2..10, "TokiPona".into())),
            })
        )
    }

    #[test]
    fn it_does_not_parse_a_milestone_without_a_time_or_a_language() {
        let src = "@ ";
        let res = parser().parse(src.to_string());
        assert!(res.is_err())
    }
}

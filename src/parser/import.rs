use chumsky::{prelude::*, text::ident};

use crate::ast::Stmt;

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    let name = just("@")
        .then(ident::<char, Simple<char>>())
        .map(|(at, id)| at.to_owned() + &id);

    let segment = name
        .or(ident())
        .or(just(".").map(|s| s.to_string()))
        .or(just("..").map(|s| s.to_string()));

    let path = just("/")
        .or_not()
        .then(
            segment
                .map_with_span(|seg, span| (span, seg))
                .separated_by(just("/"))
                .at_least(1),
        )
        .map(|(slash, segs)| (slash.is_some(), segs));

    let names = ident()
        .map_with_span(|name, span| (span, name))
        .separated_by(just(",").padded())
        .allow_trailing()
        .delimited_by(just("(").padded(), just(")").padded())
        .or(just("*").map_with_span(|star, span| vec![(span, star.to_string())]));

    just("import")
        .padded()
        .ignore_then(names)
        .then_ignore(just("from").padded())
        .then(path)
        .map(|(names, (absolute, path))| Stmt::Import {
            path,
            absolute,
            names,
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_a_relative_star_import() {
        let src = "import * from ./my/phonology";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Import {
                path: vec![
                    (14..15, ".".into()),
                    (16..18, "my".into()),
                    (19..28, "phonology".into()),
                ],
                absolute: false,
                names: vec![(7..8, "*".into())]
            })
        )
    }

    #[test]
    fn it_parses_an_absolute_star_import() {
        let src = "import * from /my/phonology";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Import {
                path: vec![(15..17, "my".into()), (18..27, "phonology".into()),],
                absolute: true,
                names: vec![(7..8, "*".into())]
            })
        )
    }

    #[test]
    fn it_parses_an_external_import() {
        let src = "import * from @core/ipa";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Import {
                path: vec![(14..19, "@core".into()), (20..23, "ipa".into()),],
                absolute: false,
                names: vec![(7..8, "*".into())]
            })
        )
    }

    #[test]
    fn it_parses_named_imports() {
        let src = "import (C, V) from @core/ipa";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Import {
                path: vec![(19..24, "@core".into()), (25..28, "ipa".into()),],
                absolute: false,
                names: vec![(8..9, "C".into()), (11..12, "V".into())]
            })
        )
    }

    #[test]
    fn it_parses_multiline_imports() {
        let src = "import (
      Place,
      Manner,
    ) from @core/ipa";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Import {
                path: vec![(47..52, "@core".into()), (53..56, "ipa".into()),],
                absolute: false,
                names: vec![(15..20, "Place".into()), (28..34, "Manner".into())]
            })
        )
    }
}

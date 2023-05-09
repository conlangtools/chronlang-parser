mod parser;
pub use parser::parse;

pub mod ast;

#[cfg(test)]
mod test {
  use super::*;

  use crate::ast::{Stmt, Spanned};

  use ariadne::{
    Report,
    Label,
    Source as SourceCode,
    ReportKind, Color,
  };
  
  use chumsky::{error::SimpleReason, prelude::Simple};

  fn display_errs(src: &str, errs: &Vec<Simple<char>>) {
    let start = errs.iter()
      .map(|err| err.span())
      .fold(src.len(), |min, cur| if cur.start < min { cur.start } else { min });

    Report::build(ReportKind::Error, (), start)
      .with_labels(
        errs.iter()
          .map(|err| {
            Label::new(err.span())
            .with_message(match err.reason() {
              SimpleReason::Unexpected => err.to_string(),
              SimpleReason::Unclosed { span: _, delimiter } => format!("Unmatched delimited {}", delimiter),
              SimpleReason::Custom(msg) => msg.clone(),
            })
            .with_color(Color::Red)
          })
      )
      .finish()
      .eprint(SourceCode::from(src.clone()))
      .unwrap();
  }

  fn _parse(src: &str) -> Result<Vec<Spanned<Stmt>>, Vec<Simple<char>>> {
    let res = parse(src);

    match res {
      Ok(ast) => Ok(ast),
      Err(errs) => {
        display_errs(&src, &errs);
        Err(errs)
      },
    }
  }

  #[test]
  fn it_works() {
    assert!(
      _parse("
        import * from @core/ipa

        lang OEng : Old English
        lang OEng < AmEng : American English
        lang OEng < RP : Received Pronunciation
        
        @ 1000, OEng
        
        - water /ˈwæ.ter/ {
          noun. liquid that forms the seas, lakes, rivers, and rain
          verb. pour or sprinkle water over a plant or area
        }
        
        @ 1940, AmEng
        
        $ [C+alveolar+stop] > [+flap] / V_V : Alveolar stops lenite to flaps intervocallically
      ").is_ok()
    )
  }
}

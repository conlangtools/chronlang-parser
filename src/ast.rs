use std::ops::Range;

pub type Span = Range<usize>;
pub type Spanned<T> = (Span, T);

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  SoundChange {
    source: Spanned<Source>,
    target: Spanned<Target>,
    environment: Option<Environment>,
    description: Option<String>,
  },
  Import(Vec<Spanned<String>>),
  Language {
    id: String,
    parent: Option<String>,
    name: Option<String>,
  },
  Word {
    gloss: String,
    pronunciation: Vec<String>,
    definitions: Vec<Definition>,
  },
  Class {
    label: String,
    class: Class,
  },
  Trait {
    label: String,
    members: Vec<TraitMember>,
  },
  Milestone {
    time: Option<Spanned<Time>>,
    language: Option<Spanned<String>>,
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
  Pattern(Pattern),
  Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
  Modification(Vec<Feature>),
  Pattern(Pattern),
  Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
  Positive(String),
  Negative(String),
}

pub type Pattern = Vec<Segment>;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
  Category(Category),
  Phonemes(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Category {
  pub base_class: Option<char>,
  pub features: Vec<Feature>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub before: Option<EnvPattern>,
  pub after: Option<EnvPattern>,
}

pub type EnvPattern = Vec<EnvElement>;

#[derive(Debug, Clone, PartialEq)]
pub enum EnvElement {
  Segment(Segment),
  SyllableBoundary,
  WordBoundary,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
  pub pos: Option<String>,
  pub definition: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Class {
  Full {
    encodes: Vec<String>,
    annotates: Vec<String>,
    phonemes: Vec<PhonemeDef>,
  },
  Category(Category),
  List(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhonemeDef {
  pub label: String,
  pub traits: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMember {
  pub labels: Vec<String>,
  pub notation: Option<String>,
  pub default: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Time {
  Instant(i64),
  Range(i64, i64),
}


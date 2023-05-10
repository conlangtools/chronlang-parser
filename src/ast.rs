use std::ops::Range;

use serde::{Serialize, Deserialize};

pub type Span = Range<usize>;
pub type Spanned<T> = (Span, T);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
  SoundChange {
    source: Spanned<Source>,
    target: Spanned<Target>,
    environment: Option<Spanned<Environment>>,
    description: Option<Spanned<String>>,
  },
  Import {
    path: Vec<Spanned<String>>,
    absolute: bool,
    names: Vec<Spanned<String>>,
  },
  Language {
    id: Spanned<String>,
    parent: Option<Spanned<String>>,
    name: Option<Spanned<String>>,
  },
  Word {
    gloss: Spanned<String>,
    pronunciation: Spanned<Vec<String>>,
    definitions: Vec<Definition>,
  },
  Class {
    label: Spanned<String>,
    encodes: Vec<Spanned<String>>,
    annotates: Vec<Spanned<String>>,
    phonemes: Vec<PhonemeDef>,
  },
  Series {
    label: Spanned<String>,
    series: Spanned<Series>,
  },
  Trait {
    label: Spanned<String>,
    members: Vec<TraitMember>,
  },
  Milestone {
    time: Option<Spanned<Time>>,
    language: Option<Spanned<String>>,
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Source {
  Pattern(Pattern),
  Empty,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Target {
  Modification(Vec<Spanned<Feature>>),
  Pattern(Pattern),
  Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Feature {
  Positive(String),
  Negative(String),
}

pub type Pattern = Vec<Segment>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Segment {
  Category(Category),
  Phonemes(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
  pub base_class: Option<Spanned<char>>,
  pub features: Vec<Spanned<Feature>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
  pub before: Option<EnvPattern>,
  pub after: Option<EnvPattern>,
}

pub type EnvPattern = Vec<EnvElement>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnvElement {
  Segment(Segment),
  SyllableBoundary,
  WordBoundary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Definition {
  pub pos: Option<Spanned<String>>,
  pub definition: Spanned<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Series {
  Category(Category),
  List(Vec<Spanned<String>>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhonemeDef {
  pub label: Spanned<String>,
  pub traits: Vec<Spanned<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitMember {
  pub labels: Vec<Spanned<String>>,
  pub notation: Option<Spanned<String>>,
  pub default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Time {
  Instant(i64),
  Range(i64, i64),
}


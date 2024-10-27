import { Category, SoundChange } from "./sound-change.ts";
import { Span, Spanned } from "./span.ts";

export type Stmt =
  | { kind: "import" } & Import
  | { kind: "language" } & Language
  | { kind: "milestone" } & Milestone
  | { kind: "trait" } & Trait
  | { kind: "class" } & Class
  | { kind: "series" } & Series
  | { kind: "word" } & Word
  | { kind: "sound-change" } & SoundChange


// IMPORT

export type BaseImport = {
  path: Spanned<string>
  names: Array<Spanned<string>>
}

export type LocalImport = BaseImport & {
  scoped: false
  absolute: boolean
}

export type ScopedImport = BaseImport & {
  scoped: true
  scope: Spanned<string>
}

export type Import = LocalImport | ScopedImport


// LANGUAGE

export type Language = {
  id: Spanned<string>
  parent: Spanned<string> | null
  name: Spanned<string> | null
}


// MILESTONE

export type Milestone = {
  time: Time | null
  language: Spanned<string>
}

export type Time = { span: Span } & (
  | { kind: "instant", time: number }
  | { kind: "range", start: number, end: number }
)


// TRAIT

export type Trait = {
  label: Spanned<string>
  members: Array<TraitMember>
}

export type TraitMember = {
  labels: Array<Spanned<string>>
  notation: Spanned<string> | null
  default: boolean
  span: Span
}


// CLASS

export type Class = {
  label: Spanned<string>
  encodes: Array<Spanned<string>>
  annotates: Array<Spanned<string>>
  phonemes: Array<PhonemeDef>
}

export type PhonemeDef = {
  label: Spanned<string>
  traits: Array<Spanned<string>>
  span: Span
}


// SERIES

export type Series = { label: Spanned<string> } & (
  | { seriesKind: "category" } & Category
  | { seriesKind: "list", phonemes: Array<Spanned<string>> }
)


// WORD

export type Word = {
  gloss: Spanned<string>,
  pronunciation: Spanned<string>
  definitions: Array<Definition>
}

export type Definition = {
  partOfSpeech: Spanned<string> | null
  text: Spanned<string>
}

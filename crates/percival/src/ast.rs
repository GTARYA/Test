//! Abstract syntax tree definitions for the Percival language.

use std::collections::{BTreeMap, BTreeSet};

/// A program translation unit in the Percival language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    /// Rules that make up the program.
    pub rules: Vec<Rule>,
    /// Imports prefixed with the `@import` specifier.
    pub imports: Vec<Import>,
}

/// Represents a single Horn clause.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// Head or implicand of the Horn clause.
    pub goal: Fact,
    /// Tail or conditional assumptions of the Horn clause.
    pub clauses: Vec<Clause>,
}

/// An element of the right-hand side of a rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Clause {
    /// Tail fact assumption of the rule.
    Fact(Fact),
    /// A raw JavaScript conditional expression between backticks.
    Expr(String),
}

/// Literal part of a Horn clause, written in terms of relations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fact {
    /// Name of the relation being referenced.
    pub name: String,
    /// Named properties of the relation.
    pub props: BTreeMap<String, Value>,
}

/// A bound or unbound value assigned to part of a relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// A simple identifier, which can be either bound or unbound.
    Id(String),
    /// A literal value, translated directly to JavaScript.
    Literal(Literal),
    /// A raw JavaScript expression between backticks.
    Expr(String),
}

/// Literal values supported by the Percival grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    /// A standard floating-point number literal.
    Number(String),
    /// A string literal, with escape sequences unevaluated.
    String(String),
    /// A boolean literal in simplest form.
    Boolean(bool),
}

/// An external import from a static JSON dataset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Import {
    /// Name of the relation being imported.
    pub name: String,
    /// Source URI of the import.
    pub uri: String,
}

impl Program {
    /// Returns the names of all relations produced by this program.
    pub fn results(&self) -> BTreeSet<String> {
        self.rules
            .iter()
            .map(|rule| rule.goal.name.clone())
            .collect()
    }

    /// Returns the names of all external relations that this program uses.
    pub fn deps(&self) -> BTreeSet<String> {
        let results = self.results();
        let imports = self.imports();
        self.rules
            .iter()
            .flat_map(|rule| {
                rule.clauses.iter().filter_map(|clause| match clause {
                    Clause::Fact(fact)
                        if !results.contains(&fact.name) && !imports.contains(&fact.name) =>
                    {
                        Some(fact.name.clone())
                    }
                    _ => None,
                })
            })
            .collect()
    }

    /// Returns the names of all external imports made by the program.
    pub fn imports(&self) -> BTreeSet<String> {
        self.imports
            .iter()
            .map(|import| import.name.clone())
            .collect()
    }
}

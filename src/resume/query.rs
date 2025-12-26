use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, ops::Not};
use toml_datetime_compat::{deserialize, serialize};

use chrono::{Datelike, NaiveDate};
use schemars::JsonSchema;

use crate::resume::schema::{Experience, When};

impl PartialEq<NaiveDate> for When {
    fn eq(&self, other: &NaiveDate) -> bool {
        match self {
            When::Range { start, end } => other >= start && other <= end,
            When::Started(start) => other >= start,
            When::Year(y) => other.year() == *y,
        }
    }
}

impl PartialEq<NaiveDate> for Experience {
    fn eq(&self, other: &NaiveDate) -> bool {
        match &self.when {
            Some(when) => *when == *other,
            None => false,
        }
    }
}

impl PartialOrd<NaiveDate> for When {
    fn partial_cmp(&self, other: &NaiveDate) -> Option<std::cmp::Ordering> {
        match self {
            When::Year(y) => y.partial_cmp(&other.year()),
            When::Range { start, .. } => start.partial_cmp(other),
            When::Started(start) => start.partial_cmp(other),
        }
    }
}

impl PartialOrd<NaiveDate> for Experience {
    fn partial_cmp(&self, other: &NaiveDate) -> Option<Ordering> {
        self.when.as_ref().and_then(|w| w.partial_cmp(other))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum Clause {
    Show(String),
    Hide(String),

    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    After(NaiveDate),
    Tagged(String),

    HideDetail(String),
    OverrideHighlights {
        target: String,
        highlights: Vec<String>,
    },
}

pub trait Query
where
    Self: Sized,
{
    fn query_one(&mut self, clause: &Clause) -> bool;

    fn query(&mut self, clauses: &[Clause]) -> bool {
        let mut show_clauses = clauses
            .iter()
            .filter(|clause| matches!(clause, Clause::Show(_)))
            .peekable();

        let mut non_show_clauses = clauses
            .iter()
            .filter(|clause| !matches!(clause, Clause::Show(_)));

        (show_clauses.peek().is_none() || show_clauses.any(|clause| self.query_one(clause)))
            && non_show_clauses.all(|clause| self.query_one(clause))
    }
}

impl Experience {
    fn contains(&self, s: &str) -> bool {
        self.name.contains(s)
            || self
                .context
                .as_ref()
                .is_some_and(|context| context.contains(s))
    }
}

impl Query for Experience {
    fn query_one(&mut self, clause: &Clause) -> bool
    where
        Self: Sized,
    {
        let show = match clause {
            Clause::Show(s) => self.contains(s),
            Clause::Hide(s) => self.contains(s).not(),
            Clause::After(d) => self.when.as_ref().is_some_and(|when| *when >= *d),
            Clause::Tagged(t) => self.tags.contains(t),
            _ => true,
        };

        if show
            && let Clause::HideDetail(s) = clause
            && self.contains(s)
        {
            self.highlights.clear();
        }

        if show
            && let Clause::OverrideHighlights { target, highlights } = clause
            && self.contains(target)
        {
            self.highlights = highlights.clone();
        }

        show
    }
}

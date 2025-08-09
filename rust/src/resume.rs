use std::{fs::File, str::FromStr};

use anyhow::{Error, Result};
use chrono::NaiveDate;
use json_resume::{Project, Resume, Work};

pub enum ResumeFilterPredicate {
    Exclude(String),
    Include(String),
    After(NaiveDate),
}

pub enum ResumeFilter {
    ProjectsFilter(ResumeFilterPredicate),
    WorkFilter(ResumeFilterPredicate),
}

pub struct ResumeSlice {
    pub filters: Vec<ResumeFilter>,
    pub resume: Resume,
}

impl From<Resume> for ResumeSlice {
    fn from(resume: Resume) -> Self {
        ResumeSlice::new(resume)
    }
}

pub trait Headline {
    fn apply(&self, predicate: &ResumeFilterPredicate) -> bool {
        match predicate {
            ResumeFilterPredicate::After(date) => {
                self.start_date().unwrap_or(NaiveDate::MIN) > *date
            }
            ResumeFilterPredicate::Exclude(value) => self.name().is_some_and(|name| name != *value),
            ResumeFilterPredicate::Include(value) => self.name().is_some_and(|name| name == *value),
        }
    }

    fn name(&self) -> Option<String>;
    fn start_date(&self) -> Option<NaiveDate>;
}

impl Headline for Work {
    fn name(&self) -> Option<String> {
        self.name.to_owned()
    }

    fn start_date(&self) -> Option<NaiveDate> {
        self.start_date
            .to_owned()
            .and_then(|s| NaiveDate::from_str(&s).ok())
    }
}

impl Headline for Project {
    fn name(&self) -> Option<String> {
        self.name.to_owned()
    }

    fn start_date(&self) -> Option<NaiveDate> {
        self.start_date
            .to_owned()
            .and_then(|s| NaiveDate::from_str(&s).ok())
    }
}

impl ResumeSlice {
    pub fn projects(mut self, predicates: impl IntoIterator<Item = ResumeFilterPredicate>) -> Self {
        self.filters
            .extend(predicates.into_iter().map(ResumeFilter::ProjectsFilter));
        self
    }

    pub fn work(mut self, predicates: impl IntoIterator<Item = ResumeFilterPredicate>) -> Self {
        self.filters
            .extend(predicates.into_iter().map(ResumeFilter::WorkFilter));
        self
    }

    pub fn new(resume: Resume) -> ResumeSlice {
        ResumeSlice {
            filters: Vec::new(),
            resume,
        }
    }

    pub fn render_document(self) -> Result<String> {
        Err(Error::msg("Unimplemented!"))
    }
}

use anyhow::{Error, Result};
use bon::Builder;
use chrono::NaiveDate;
use json_resume::Resume;

use crate::resume::resume_copy_builder::State;

pub enum ResumeFilterPredicate {
    Include(String),
    Exclude(String),
    After(NaiveDate),
}

pub enum ResumeFilter {
    ProjectsFilter(ResumeFilterPredicate),
    WorkFilter(ResumeFilterPredicate),
}

pub enum ResumeSection {
    Skills,
    Experiences,
    Projects,
    Education,
}

#[derive(Builder)]
pub struct ResumeCopy {
    #[builder(field)]
    pub filters: Vec<ResumeFilter>,

    #[builder(into)]
    pub sections: Vec<ResumeSection>,

    pub resume: Resume,
}

impl<S: State> ResumeCopyBuilder<S> {
    pub fn filter(mut self, filter: ResumeFilter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn filters(mut self, filters: impl IntoIterator<Item = ResumeFilter>) -> Self {
        self.filters.extend(filters);
        self
    }

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
}

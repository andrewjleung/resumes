use std::str::FromStr;

use chrono::NaiveDate;
use json_resume::{Project, Resume, Work};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ResumeFilterPredicate {
    Exclude(String),
    Include(String),
    After(NaiveDate),
}

pub trait Headline {
    fn apply(&self, predicate: &ResumeFilterPredicate) -> bool {
        match predicate {
            ResumeFilterPredicate::After(date) => {
                self.start_date().unwrap_or(NaiveDate::MIN) >= *date
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

pub struct ResumeSlice {
    pub work_filters: Vec<ResumeFilterPredicate>,
    pub project_filters: Vec<ResumeFilterPredicate>,
    pub resume: Resume,
}

impl From<Resume> for ResumeSlice {
    fn from(resume: Resume) -> Self {
        ResumeSlice::new(resume)
    }
}

impl From<ResumeSlice> for Resume {
    fn from(resume_slice: ResumeSlice) -> Self {
        let mut resume = resume_slice.resume;

        resume
            .work
            .retain(|work| resume_slice.work_filters.iter().all(|f| work.apply(f)));

        resume.projects.retain(|project| {
            resume_slice
                .project_filters
                .iter()
                .all(|f| project.apply(f))
        });

        resume
    }
}

impl ResumeSlice {
    pub fn projects(mut self, predicates: impl IntoIterator<Item = ResumeFilterPredicate>) -> Self {
        self.project_filters.extend(predicates);
        self
    }

    pub fn work(mut self, predicates: impl IntoIterator<Item = ResumeFilterPredicate>) -> Self {
        self.work_filters.extend(predicates);
        self
    }

    pub fn new(resume: Resume) -> ResumeSlice {
        ResumeSlice {
            work_filters: Vec::new(),
            project_filters: Vec::new(),
            resume,
        }
    }

    pub fn apply_slice(self) -> Resume {
        Resume::from(self)
    }
}

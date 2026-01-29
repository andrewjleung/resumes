use crate::prelude::*;
use crate::resume::schema::Resume;
use crate::{command::Run, config::Config};
use clap::{Args, ValueEnum};
use schemars::schema_for;

#[derive(ValueEnum, Clone)]
enum SchemaTarget {
    Config,
    Resume,
}

#[derive(Args)]
pub struct Schema {
    target: SchemaTarget,
}

impl Run for Schema {
    fn run(self) -> Result<()> {
        let schema = match self.target {
            SchemaTarget::Config => schema_for!(Config),
            SchemaTarget::Resume => schema_for!(Resume),
        };

        println!(
            "{}",
            serde_json::to_string_pretty(&schema).context("failed to convert schema into JSON")?
        );

        Ok(())
    }
}

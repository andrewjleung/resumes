use std::collections::HashMap;

use bon::Builder;
use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_datetime_compat::{deserialize, serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder, JsonSchema)]
pub struct Resume {
    pub profile: Profile,
    pub contact: Option<Contact>,

    #[serde(default)]
    pub education: Vec<Education>,

    #[serde(default)]
    pub socials: Vec<Social>,

    #[serde(default)]
    pub skills: HashMap<String, Vec<String>>,

    #[serde(default)]
    pub experiences: Vec<Experience>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder, JsonSchema)]
pub struct Profile {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub profile_picture_url: Option<String>,
    pub headline: Option<String>,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder, JsonSchema)]
pub struct Contact {
    pub personal_email: String,
    pub work_email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder, JsonSchema)]
pub struct Social {
    pub network: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder, JsonSchema)]
pub struct Experience {
    pub kind: String,
    pub name: String,
    pub context: Option<String>,
    pub summary: Option<String>,
    pub when: Option<When>,
    pub location: Option<Location>,
    pub url: Option<String>,

    #[serde(default)]
    pub highlights: Vec<String>,

    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder, JsonSchema)]
pub struct Education {
    pub institution: String,
    pub when: Option<When>,
    pub location: Option<Location>,
    pub url: Option<String>,
    pub area: Option<String>,
    pub kind: Option<String>,
    pub score: Option<String>,

    #[serde(default)]
    pub highlights: Vec<String>,

    #[serde(default)]
    pub courses: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum When {
    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    Date(NaiveDate),

    Range {
        #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
        start: NaiveDate,

        #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
        end: NaiveDate,
    },

    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    Started(NaiveDate),
    Year(i32),
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum Location {
    CityState(String, String),
    Address(Address),
    Remote,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder, JsonSchema)]
pub struct Address {
    pub name: Option<String>,
    pub country_code: String,
    pub region: String,
    pub city: String,
    pub postal_code: Option<String>,
    pub address: String,
    pub address_second_line: String,
}

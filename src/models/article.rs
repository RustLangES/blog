use std::collections::HashMap;

use chrono::{Locale, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub github_user: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub content: String,
    #[serde(deserialize_with = "parse_date")]
    pub date: String,
    pub social: HashMap<String, String>,
}

fn parse_date<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
    D::Error: serde::de::Error,
{
    let error = Err(serde::de::Error::custom("Error in date parsing"));
    let date_str: String = Deserialize::deserialize(de)?;
    let mut splitted = date_str.split('-');
    let Ok(year) = splitted.next().unwrap_or_default().parse::<i32>() else {
        return error;
    };
    let Ok(month) = splitted.next().unwrap_or_default().parse::<u32>() else {
        return error;
    };
    let Ok(day) = splitted.next().unwrap_or_default().parse::<u32>() else {
        return error;
    };
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    Ok(date
        .format_localized("%e de %B del %Y", Locale::es_ES)
        .to_string())
}

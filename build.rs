use std::process::Command;
use std::{fs, io};

use csv::Reader;
use serde::Deserialize;

const DATA_FILE_PREFIX: &'static str = "use std::collections::HashMap;

use slab::Slab;

use crate::schema::{Issue, IssueKind, Score, ScoreKind, State};

pub struct Data;

impl Data {
    pub fn seed() -> (Slab<State>, HashMap<&'static str, usize>) {
        let mut states = Slab::new();
        let mut state_data = HashMap::new();
";
const DATA_FILE_SUFFIX: &'static str = "        (states, state_data)
    }
}";

fn main() -> io::Result<()> {
    //println!("cargo:rerun-if-changed=data/states.csv");
    Generator::generate()
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Row {
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "State Abbreviation")]
    state_abbreviation: String,
    #[serde(rename = "Region")]
    region: String,
    #[serde(rename = "Division")]
    division: String,
    #[serde(rename = "Population")]
    population: usize,
    #[serde(rename = "Score")]
    score: String,
    #[serde(rename = "Anti-LGBTQ Bills")]
    anti_lgbtq_bills: String,
    #[serde(rename = "Transgender Healthcare")]
    transgender_healthcare: String,
    #[serde(rename = "Transgender Healthcare Value")]
    transgender_healthcare_value: i8,
    #[serde(rename = "School Anti-Bullying")]
    school_anti_bullying: String,
    #[serde(rename = "School Anti-Bullying Value")]
    school_anti_bullying_value: i8,
    #[serde(rename = "Public Accommodations")]
    public_accommodations: String,
    #[serde(rename = "Public Accommodations Value")]
    public_accommodations_value: i8,
    #[serde(rename = "Marriage Equality & Relationship Recognition")]
    marriage_equality: String,
    #[serde(rename = "Marriage Equality & Relationship Recognition Value")]
    marriage_equality_value: i8,
    #[serde(rename = "Housing")]
    housing: String,
    #[serde(rename = "Housing Value")]
    housing_value: i8,
    #[serde(rename = "Hate Crimes")]
    hate_crimes: String,
    #[serde(rename = "Hate Crimes Value")]
    hate_crimes_value: i8,
    #[serde(rename = "Gender Marker Updates on Identification Documents")]
    gender_marker_updates: String,
    #[serde(rename = "Gender Marker Updates on Identification Documents Value")]
    gender_marker_updates_value: i8,
    #[serde(rename = "Employment")]
    employment: String,
    #[serde(rename = "Employment Value")]
    employment_value: i8,
    #[serde(rename = "Education")]
    education: String,
    #[serde(rename = "Education Value")]
    education_value: i8,
    #[serde(rename = "Discrimination in Child Welfare Services")]
    discrimination_in_child_welfare: String,
    #[serde(rename = "Discrimination in Child Welfare Services Value")]
    discrimination_in_child_welfare_value: i8,
    #[serde(rename = "Anti-Conversion Therapy")]
    anti_conversion_therapy: String,
    #[serde(rename = "Anti-Conversion Therapy Value")]
    anti_conversion_therapy_value: i8,
}

fn score_kind_from_str(string: &str) -> &'static str {
    match string {
        "High Priority to Achieve Basic Equality" => "HighPriority",
        "Building Equality" => "Building",
        "Solidifying Equality" => "Solidifying",
        "Working Toward Innovative Equality" => "Innovative",
        _ => "",
    }
}

struct Generator;
impl Generator {
    fn generate() -> io::Result<()> {
        let mut state_reader = Reader::from_path("./data/states.csv")?;
        let iter = state_reader.deserialize();
        let mut data = String::new();

        for result in iter {
            let record: Row = result?;
            let identifier = record
                .state
                .replace(",", "")
                .replace(" ", "_")
                .to_lowercase();
            let state_1 = format!(
                "        let {} = states.insert(State {{
            id: \"{}\",
            name: \"{}\",
            region: \"{}\",
            district: \"{}\",
            score: Score {{
                description: \"{}\".to_string(),
                kind: ScoreKind::{},
            }},\n",
                &identifier,
                &record.state_abbreviation,
                &record.state,
                &record.region,
                &record.division,
                &record.score,
                score_kind_from_str(&record.score)
            );

            data.push_str(&state_1);

            let issues = vec![
                (
                    "Transgender Healthcare",
                    "TransgenderHealthcare",
                    &record.transgender_healthcare,
                    &record.transgender_healthcare_value,
                ),
                (
                    "School Anti-Bullying",
                    "SchoolAntiBullying",
                    &record.school_anti_bullying,
                    &record.school_anti_bullying_value,
                ),
                (
                    "Public Accommodations",
                    "PublicAccommodations",
                    &record.public_accommodations,
                    &record.public_accommodations_value,
                ),
                (
                    "Marriage Equality & Relationship Recognition",
                    "MarriageEquality",
                    &record.marriage_equality,
                    &record.marriage_equality_value,
                ),
                ("Housing", "Housing", &record.housing, &record.housing_value),
                (
                    "Hate Crimes",
                    "HateCrimes",
                    &record.hate_crimes,
                    &record.hate_crimes_value,
                ),
                (
                    "Gender Marker Updates on Identification Documents",
                    "GenderMarkerUpdatesOnIdentification",
                    &record.gender_marker_updates,
                    &record.gender_marker_updates_value,
                ),
                (
                    "Employment",
                    "Employment",
                    &record.employment,
                    &record.employment_value,
                ),
                (
                    "Education",
                    "Education",
                    &record.education,
                    &record.education_value,
                ),
                (
                    "Discrimination in Child Welfare Services",
                    "DiscriminationInChildWelfare",
                    &record.discrimination_in_child_welfare,
                    &record.discrimination_in_child_welfare_value,
                ),
                (
                    "Anti-Conversion Therapy",
                    "AntiConversionTherapy",
                    &record.anti_conversion_therapy,
                    &record.anti_conversion_therapy_value,
                ),
            ];

            let iss_prefix = "            issues: vec![\n";
            let iss_suffix = format!(
                "],
        }});
        state_data.insert(\"{}\", {});",
                &record.state_abbreviation, &identifier
            );
            let mut issue_vec = String::new();
            issue_vec.push_str(iss_prefix);
            for issue in issues {
                let issue_code = format!(
                    "                Issue {{
                    name: \"{}\".to_string(),
                    kind: IssueKind::{},
                    description: \"{}\".to_string(),
                    value: {},
                }},\n",
                    issue.0,
                    issue.1,
                    issue.2.replace("\"", "\\\""),
                    issue.3
                );

                issue_vec.push_str(&issue_code);
            }
            issue_vec.push_str(&iss_suffix);
            data.push_str(&issue_vec);
            data.push_str("\n");
        }

        let contents = format!("{}{}{}", DATA_FILE_PREFIX, data, DATA_FILE_SUFFIX);

        fs::write("./src/generated.rs", &contents)?;

        //Command::new("rustfmt")
        //.args(&["--emit", "files", "./src/generated.rs"])
        //.output()?;

        Ok(())
    }
}

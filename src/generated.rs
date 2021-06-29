use std::collections::HashMap;

use slab::Slab;

use crate::schema::{Issue, IssueKind, Score, ScoreKind, State};

pub struct Data;

impl Data {
    pub fn seed() -> (Slab<State>, HashMap<&'static str, usize>) {
        let mut states = Slab::new();
        let mut state_data = HashMap::new();
        let alabama = states.insert(State {
            id: "AL",
            name: "Alabama",
            region: "South",
            district: "East South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Restrict the inclusion of LGBTQ topics in schools".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Discrimination is allowed by private agencies which do not receive taxpayer funds".to_string(),
                    value: -5,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("AL", alabama);
        let alaska = states.insert(State {
            id: "AK",
            name: "Alaska",
            region: "West",
            district: "Pacific",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("AK", alaska);
        let arizona = states.insert(State {
            id: "AZ",
            name: "Arizona",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 5,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("AZ", arizona);
        let arkansas = states.insert(State {
            id: "AR",
            name: "Arkansas",
            region: "South",
            district: "West South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("AR", arkansas);
        let california = states.insert(State {
            id: "CA",
            name: "California",
            region: "West",
            district: "Pacific",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("CA", california);
        let colorado = states.insert(State {
            id: "CO",
            name: "Colorado",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("CO", colorado);
        let connecticut = states.insert(State {
            id: "CT",
            name: "Connecticut",
            region: "Northeast",
            district: "New England",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("CT", connecticut);
        let delaware = states.insert(State {
            id: "DE",
            name: "Delaware",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("DE", delaware);
        let florida = states.insert(State {
            id: "FL",
            name: "Florida",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 5,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("FL", florida);
        let georgia = states.insert(State {
            id: "GA",
            name: "Georgia",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only"
                        .to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
            ],
        });
        state_data.insert("GA", georgia);
        let hawaii = states.insert(State {
            id: "HI",
            name: "Hawaii",
            region: "West",
            district: "Pacific",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("HI", hawaii);
        let idaho = states.insert(State {
            id: "ID",
            name: "Idaho",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("ID", idaho);
        let illinois = states.insert(State {
            id: "IL",
            name: "Illinois",
            region: "Midwest",
            district: "East North Central",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("IL", illinois);
        let indiana = states.insert(State {
            id: "IN",
            name: "Indiana",
            region: "Midwest",
            district: "East North Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination against public employees based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("IN", indiana);
        let iowa = states.insert(State {
            id: "IA",
            name: "Iowa",
            region: "Midwest",
            district: "West North Central",
            score: Score {
                description: "Solidifying Equality".to_string(),
                kind: ScoreKind::Solidifying,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("IA", iowa);
        let kansas = states.insert(State {
            id: "KS",
            name: "Kansas",
            region: "Midwest",
            district: "West North Central",
            score: Score {
                description: "Building Equality".to_string(),
                kind: ScoreKind::Building,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 5,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows agencies to discriminate against potential parents".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("KS", kansas);
        let kentucky = states.insert(State {
            id: "KY",
            name: "Kentucky",
            region: "South",
            district: "East South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination against public employees based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("KY", kentucky);
        let louisiana = states.insert(State {
            id: "LA",
            name: "Louisiana",
            region: "South",
            district: "West South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Restrict the inclusion of LGBTQ topics in schools".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only"
                        .to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
            ],
        });
        state_data.insert("LA", louisiana);
        let maine = states.insert(State {
            id: "ME",
            name: "Maine",
            region: "Northeast",
            district: "New England",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("ME", maine);
        let maryland = states.insert(State {
            id: "MD",
            name: "Maryland",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("MD", maryland);
        let massachusetts = states.insert(State {
            id: "MA",
            name: "Massachusetts",
            region: "Northeast",
            district: "New England",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("MA", massachusetts);
        let michigan = states.insert(State {
            id: "MI",
            name: "Michigan",
            region: "Midwest",
            district: "East North Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination against public employees based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows agencies to refuse to work with potential parents and children".to_string(),
                    value: -2,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("MI", michigan);
        let minnesota = states.insert(State {
            id: "MN",
            name: "Minnesota",
            region: "Midwest",
            district: "West North Central",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("MN", minnesota);
        let mississippi = states.insert(State {
            id: "MS",
            name: "Mississippi",
            region: "South",
            district: "East South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Restrict the inclusion of LGBTQ topics in schools".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows sweeping anti-LGBTQ discrimination that includes allowing agencies to refuse to work with potential parents and children and agencies can deny children services to which the agency objects, including refusing to allow transgender people access".to_string(),
                    value: -4,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("MS", mississippi);
        let missouri = states.insert(State {
            id: "MO",
            name: "Missouri",
            region: "Midwest",
            district: "West North Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Prevents school districts from specifically protecting LGBTQ students".to_string(),
                    value: -2,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination against public employees based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("MO", missouri);
        let montana = states.insert(State {
            id: "MT",
            name: "Montana",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination against public employees based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("MT", montana);
        let nebraska = states.insert(State {
            id: "NE",
            name: "Nebraska",
            region: "Midwest",
            district: "West North Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 5,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("NE", nebraska);
        let nevada = states.insert(State {
            id: "NV",
            name: "Nevada",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("NV", nevada);
        let new_hampshire = states.insert(State {
            id: "NH",
            name: "New Hampshire",
            region: "Northeast",
            district: "New England",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("NH", new_hampshire);
        let new_jersey = states.insert(State {
            id: "NJ",
            name: "New Jersey",
            region: "Northeast",
            district: "Middle Atlantic",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("NJ", new_jersey);
        let new_mexico = states.insert(State {
            id: "NM",
            name: "New Mexico",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "Address discrimination against students based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("NM", new_mexico);
        let new_york = states.insert(State {
            id: "NY",
            name: "New York",
            region: "Northeast",
            district: "Middle Atlantic",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("NY", new_york);
        let north_carolina = states.insert(State {
            id: "NC",
            name: "North Carolina",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination against public employees based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("NC", north_carolina);
        let north_dakota = states.insert(State {
            id: "ND",
            name: "North Dakota",
            region: "Midwest",
            district: "West North Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 5,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows agencies to discriminate against potential parents".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("ND", north_dakota);
        let ohio = states.insert(State {
            id: "OH",
            name: "Ohio",
            region: "Midwest",
            district: "East North Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination against public employees based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("OH", ohio);
        let oklahoma = states.insert(State {
            id: "OK",
            name: "Oklahoma",
            region: "West",
            district: "West South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Restrict the inclusion of LGBTQ topics in schools".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows agencies to discriminate against potential parents"
                        .to_string(),
                    value: -1,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
            ],
        });
        state_data.insert("OK", oklahoma);
        let oregon = states.insert(State {
            id: "OR",
            name: "Oregon",
            region: "West",
            district: "Pacific",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("OR", oregon);
        let pennsylvania = states.insert(State {
            id: "PA",
            name: "Pennsylvania",
            region: "Northeast",
            district: "Middle Atlantic",
            score: Score {
                description: "Building Equality".to_string(),
                kind: ScoreKind::Building,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 5,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("PA", pennsylvania);
        let rhode_island = states.insert(State {
            id: "RI",
            name: "Rhode Island",
            region: "Northeast",
            district: "New England",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("RI", rhode_island);
        let south_carolina = states.insert(State {
            id: "SC",
            name: "South Carolina",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Restrict the inclusion of LGBTQ topics in schools".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
            ],
        });
        state_data.insert("SC", south_carolina);
        let south_dakota = states.insert(State {
            id: "SD",
            name: "South Dakota",
            region: "Midwest",
            district: "West North Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Prevents school districts from specifically protecting LGBTQ students".to_string(),
                    value: -2,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows sweeping anti-LGBTQ discrimination that includes allowing agencies to refuse to work with potential parents and children and agencies can deny children services to which the agency objects, including refusing to allow transgender people access".to_string(),
                    value: -4,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("SD", south_dakota);
        let tennessee = states.insert(State {
            id: "TN",
            name: "Tennessee",
            region: "South",
            district: "East South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only"
                        .to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows agencies to discriminate against potential parents"
                        .to_string(),
                    value: -1,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
            ],
        });
        state_data.insert("TN", tennessee);
        let texas = states.insert(State {
            id: "TX",
            name: "Texas",
            region: "South",
            district: "West South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Restrict the inclusion of LGBTQ topics in schools".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 5,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows sweeping anti-LGBTQ discrimination that includes allowing agencies to refuse to work with potential parents and children and agencies can deny children services to which the agency objects, including refusing to allow transgender people access".to_string(),
                    value: -4,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("TX", texas);
        let utah = states.insert(State {
            id: "UT",
            name: "Utah",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "Building Equality".to_string(),
                kind: ScoreKind::Building,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("UT", utah);
        let vermont = states.insert(State {
            id: "VT",
            name: "Vermont",
            region: "Northeast",
            district: "New England",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("VT", vermont);
        let virginia = states.insert(State {
            id: "VA",
            name: "Virginia",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "Solidifying Equality".to_string(),
                kind: ScoreKind::Solidifying,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "Allows agencies to discriminate against potential parents".to_string(),
                    value: -1,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("VA", virginia);
        let washington = states.insert(State {
            id: "WA",
            name: "Washington",
            region: "West",
            district: "Pacific",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation and gender identity".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("WA", washington);
        let washington_dc = states.insert(State {
            id: "DC",
            name: "Washington, DC",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "Working Toward Innovative Equality".to_string(),
                kind: ScoreKind::Innovative,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "Address harassment and/or bullying of students based on sexual orientation and gender identity".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 3,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Enforcement authorities are accepting complaints on the basis of sexual orientation and gender identity because the state has adopted the Bostock rationale into state law".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on both birth certificates and driver's licenses".to_string(),
                    value: 2,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "Prohibit discrimination based on sexual orientation and gender identity".to_string(),
                    value: 6,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "State Protects Youth From So-Called \"Conversion Therapy\"".to_string(),
                    value: 1,
                },
],
        });
        state_data.insert("DC", washington_dc);
        let west_virginia = states.insert(State {
            id: "WV",
            name: "West Virginia",
            region: "South",
            district: "South Atlantic",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "Facilitate gender marker update on driver's licenses only"
                        .to_string(),
                    value: 1,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
            ],
        });
        state_data.insert("WV", west_virginia);
        let wisconsin = states.insert(State {
            id: "WI",
            name: "Wisconsin",
            region: "Midwest",
            district: "East North Central",
            score: Score {
                description: "Building Equality".to_string(),
                kind: ScoreKind::Building,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "Bans insurance exclusions for transgender healthcare and also has protections for transgender healthcare in state Medicaid".to_string(),
                    value: 2,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "Prohibit discrimination based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "Prohibit discrimination based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "Address hate or bias crimes based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "Address discrimination against students based on sexual orientation only".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
],
        });
        state_data.insert("WI", wisconsin);
        let wyoming = states.insert(State {
            id: "WY",
            name: "Wyoming",
            region: "West",
            district: "Mountain",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![
                Issue {
                    name: "Transgender Healthcare".to_string(),
                    kind: IssueKind::TransgenderHealthcare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "School Anti-Bullying".to_string(),
                    kind: IssueKind::SchoolAntiBullying,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Public Accommodations".to_string(),
                    kind: IssueKind::PublicAccommodations,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Marriage Equality & Relationship Recognition".to_string(),
                    kind: IssueKind::MarriageEquality,
                    description: "Issue marriage licenses to same-sex couples".to_string(),
                    value: 1,
                },
                Issue {
                    name: "Housing".to_string(),
                    kind: IssueKind::Housing,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Hate Crimes".to_string(),
                    kind: IssueKind::HateCrimes,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Gender Marker Updates on Identification Documents".to_string(),
                    kind: IssueKind::GenderMarkerUpdatesOnIdentification,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Employment".to_string(),
                    kind: IssueKind::Employment,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Education".to_string(),
                    kind: IssueKind::Education,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Discrimination in Child Welfare Services".to_string(),
                    kind: IssueKind::DiscriminationInChildWelfare,
                    description: "None".to_string(),
                    value: 0,
                },
                Issue {
                    name: "Anti-Conversion Therapy".to_string(),
                    kind: IssueKind::AntiConversionTherapy,
                    description: "None".to_string(),
                    value: 0,
                },
            ],
        });
        state_data.insert("WY", wyoming);
        (states, state_data)
    }
}

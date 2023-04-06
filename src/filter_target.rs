use std::collections::HashMap;
use std::{fs::File, io::BufReader, path::Path};

use csv::{QuoteStyle, WriterBuilder};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use serde_with::formats::PreferMany;
use serde_with::serde_as;

#[derive(Serialize, Deserialize)]
pub struct Target {
    #[serde(default)]
    pub polypeptide: Option<PolypeptideOrVec>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum PolypeptideOrVec {
    None,
    One(Polypeptide),
    Many(Vec<Polypeptide>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Polypeptide {
    #[serde(rename = "gene-name")]
    pub gene_name: Value,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Targets {
    #[serde_as(as = "serde_with::OneOrMany<_, PreferMany>")]
    pub target: Vec<Target>,
}

#[derive(Serialize, Deserialize)]
pub struct Drug {
    pub name: String,
    pub toxicity: Value,
    #[serde(deserialize_with = "target_or_object")]
    pub targets: Targets,
}

fn target_or_object<'de, D>(deserializer: D) -> Result<Targets, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TargetOrObject {
        T(Targets),
        N(HashMap<String, String>),
    }

    Ok(match TargetOrObject::deserialize(deserializer)? {
        TargetOrObject::T(v) => v, // Ignoring parsing errors
        TargetOrObject::N(_) => Targets { target: vec![] },
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrugCsv {
    name: String,
    gene: String,
    toxicity: String,
}

fn save_to_file<T: Serialize>(
    name: &str,
    v: &Vec<T>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut writer = WriterBuilder::new()
        .quote_style(QuoteStyle::Necessary)
        .from_path(name)?;

    for person in v {
        writer.serialize(person)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn read_json<P: AsRef<Path>>(path: P) -> DrugCsv {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let drug: Drug = serde_json::from_reader(reader).unwrap();

    let vv: Vec<Target> = drug.targets.target.into();

    let genes = vv
        .into_iter()
        .filter_map(|f| match f.polypeptide {
            Some(p) => match p {
                PolypeptideOrVec::None => None,
                PolypeptideOrVec::One(pp) => match pp.gene_name.clone() {
                    Value::String(s) => Some(s),
                    _ => None,
                },
                PolypeptideOrVec::Many(m) => {
                    let gg = m
                        .iter()
                        .filter_map(|p| match p.gene_name.clone() {
                            Value::String(s) => Some(s),
                            _ => None,
                        })
                        .collect::<Vec<String>>()
                        .join("|");

                    Some(gg)
                }
            },
            None => None,
        })
        .collect::<Vec<String>>()
        .join("|");

    DrugCsv {
        name: drug.name.clone(),
        gene: genes,
        toxicity: match drug.toxicity.clone() {
            Value::String(e) => e.replace("\r\n", "").replace("\n", ""),
            _ => "".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(id: usize) {
        let path = format!("data/json/{}.json", id);
        log::info!("start parse {}", &path);
        log::info!("DrugCsv: {:?}", serde_json::to_string(&read_json(&path)));
    }

    #[test]
    fn test_read_file() {
        crate::config::init_config();
        check(0);
        check(1);
        check(4080);
        check(14203);
        check(1273);
        check(11710);
    }

    #[test]
    fn test_name() {
        crate::config::init_config();
        let drugs: Vec<DrugCsv> = std::fs::read_dir("data/json")
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                log::info!("path = {:?}", &path);
                read_json(&path)
            })
            .collect();

        let _ = save_to_file("data/target_gene.csv", &drugs);
    }
}

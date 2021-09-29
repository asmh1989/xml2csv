use jwalk::WalkDirGeneric;
use mongodb::bson::{self, Document};
use serde::{Deserialize, Serialize};

use crate::model::{Drugbank, Property};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    #[serde(rename = "type")]
    pub type_filed: String,
    pub drugbank_id: String,
    pub smiles: String,
    pub inchi: String,
    pub inchi_key: String,
    pub groups: Vec<String>,
    pub experimental_properties: Vec<Property>,
}

impl Filter {
    pub fn new(
        type_filed: String,
        smiles: String,
        drugbank_id: String,
        inchi: String,
        experimental_properties: Vec<Property>,
        inchi_key: String,
        groups: Vec<String>,
    ) -> Self {
        Self {
            id: None,
            type_filed,
            smiles,
            experimental_properties,
            inchi,
            inchi_key,
            drugbank_id,
            groups,
        }
    }

    pub fn document(&self) -> Result<Document, String> {
        match bson::to_bson(&self) {
            Ok(d) => return Ok(d.as_document().unwrap().clone()),
            Err(e) => {
                log::info!("to_bson err {}", e);
                return Err(format!("to_bson error : {}", e));
            }
        };
    }

    pub fn save_db(&self) -> Result<(), String> {
        let doc = match bson::to_bson(&self) {
            Ok(d) => d.as_document().unwrap().clone(),
            Err(e) => {
                log::info!("to_bson err {}", e);
                return Err(format!("to_bson error : {}", e));
            }
        };

        if let Err(e) = crate::db::Db::save(
            "filter_properties",
            mongodb::bson::doc! {"drugbankId" :self.drugbank_id.clone()},
            doc.clone(),
        ) {
            log::info!("db save error {} ", e);
            return Err(format!("db save error {} ", e));
        }
        Ok(())
    }
}

pub fn start_filter(path: &str) {
    let mut files = Vec::<String>::new();
    // let shell = crate::shell::Shell::new(".");

    log::info!("start find json files...");
    for entry in WalkDirGeneric::<((), ())>::new(path).process_read_dir(move |_, _, _, _| {}) {
        let d = entry.unwrap().path();
        if let Some(k) = d.extension() {
            if k == "json" {
                let f = d
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap()
                    .to_string();
                // if let Ok(_) = shell.run(&format!("cat {} | grep \"small molecule\"", f.clone())) {
                files.push(f);
                // }
            }
        }
    }

    log::info!("start parse json files...");

    files.into_iter().for_each(|f| get_drug(&f))
}

fn get_drug(f: &str) {
    match parse_json(&f) {
        Ok(v) => {
            let mut inchi = "".to_string();
            let mut inchi_key = "".to_string();
            let mut smiles = "".to_string();
            let mut experimental_properties = vec![];
            let mut drukbank_id = "".to_string();
            let mut groups = vec![];

            if v.type_field == "small molecule" {
                if let Some(p) = v.calculated_properties {
                    if let Some(p) = p.property {
                        match p {
                            crate::model::PropertyEnum::ARR(p) => {
                                p.iter().for_each(|f| match &f.kind[..] {
                                    "SMILES" => {
                                        smiles = f.value.as_str().unwrap().to_string();
                                    }
                                    "InChI" => {
                                        inchi = f.value.as_str().unwrap().to_string();
                                    }
                                    "InChIKey" => {
                                        inchi_key = f.value.as_str().unwrap().to_string();
                                    }
                                    _ => {}
                                })
                            }
                            crate::model::PropertyEnum::OO(_) => {}
                        }
                    }
                }

                if let Some(p) = v.experimental_properties {
                    if let Some(p) = p.property {
                        match p {
                            crate::model::PropertyEnum::ARR(p) => {
                                experimental_properties.extend(p.clone().into_iter())
                            }
                            crate::model::PropertyEnum::OO(p) => {
                                experimental_properties.push(p.clone())
                            }
                        }
                    }
                }

                if let Some(obj) = v.drugbank_id.as_object() {
                    if let Some(p) = obj.get("#text") {
                        drukbank_id = p.as_str().unwrap().to_string();
                    }
                } else if let Some(array) = v.drugbank_id.as_array() {
                    array.iter().for_each(|f| {
                        if let Some(obj) = f.as_object() {
                            if let Some(p) = obj.get("#text") {
                                drukbank_id = p.as_str().unwrap().to_string();
                            }
                        }
                    });
                }

                if let Some(g) = v.groups.group.as_array() {
                    groups.extend(
                        g.clone()
                            .into_iter()
                            .map(|f| f.as_str().unwrap().to_string()),
                    );
                } else if let Some(g) = v.groups.group.as_str() {
                    groups.push(g.to_string());
                }

                let drug = Filter::new(
                    v.type_field,
                    smiles,
                    drukbank_id,
                    inchi,
                    experimental_properties,
                    inchi_key,
                    groups,
                );
                let _ = drug.save_db();

                // log::info!("drugbank:{:?}", serde_json::to_string(&drug).unwrap());
            }
        }
        Err(err) => {
            log::error!("file = {:?}, parse json error : {:?}", &f, err)
        }
    }
}

pub fn parse_json(file: &str) -> Result<Drugbank, String> {
    let file = std::fs::read(file).map_err(|f| f.to_string())?;
    let str = unsafe { String::from_utf8_unchecked(file) };
    // let str = std::fs::read_to_string(file).unwrap();
    // let str = str.replace("#text", "text");
    let json: Drugbank = serde_json::from_str(&str).map_err(|f| f.to_string())?;

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        crate::config::init_config();
        crate::db::init_db("mongodb://192.168.2.25:27017");

        start_filter("data/json");
    }

    #[test]
    fn test_get_drug() {
        crate::config::init_config();
        crate::db::init_db("mongodb://192.168.2.25:27017");
        get_drug("data/json/7917.json");
        get_drug("data/json/7372.json");
    }
}

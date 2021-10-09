use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drugbank {
    #[serde(rename = "@created")]
    pub created: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    // #[serde(rename = "@updated")]
    // pub updated: String,
    // pub absorption: Absorption,
    // #[serde(rename = "affected-organisms")]
    // pub affected_organisms: AffectedOrganisms,
    // #[serde(rename = "ahfs-codes")]
    // pub ahfs_codes: AhfsCodes,
    // #[serde(rename = "atc-codes")]
    // pub atc_codes: AtcCodes,
    #[serde(rename = "average-mass")]
    pub average_mass: Option<f64>,
    #[serde(rename = "calculated-properties")]
    pub calculated_properties: Option<Properties>,
    // pub carriers: Carriers,
    // #[serde(rename = "cas-number")]
    // pub cas_number: String,
    // pub categories: Categories,
    // pub clearance: Clearance,
    // pub description: String,
    // pub dosages: Dosages,
    // #[serde(rename = "drug-interactions")]
    // pub drug_interactions: DrugInteractions,
    #[serde(rename = "drugbank-id")]
    pub drugbank_id: ::serde_json::Value,
    // pub enzymes: Enzymes,
    #[serde(rename = "experimental-properties")]
    pub experimental_properties: Option<Properties>,
    // #[serde(rename = "external-identifiers")]
    // pub external_identifiers: ExternalIdentifiers,
    // #[serde(rename = "external-links")]
    // pub external_links: ExternalLinks,
    // #[serde(rename = "food-interactions")]
    // pub food_interactions: FoodInteractions,
    // #[serde(rename = "general-references")]
    // pub general_references: GeneralReferences,
    pub groups: Groups,
    // #[serde(rename = "half-life")]
    // pub half_life: HalfLife,
    // pub indication: Indication,
    // #[serde(rename = "international-brands")]
    // pub international_brands: InternationalBrands,
    // pub manufacturers: Manufacturers,
    // #[serde(rename = "mechanism-of-action")]
    // pub mechanism_of_action: MechanismOfAction,
    // pub metabolism: Metabolism,
    // pub mixtures: Mixtures,
    // #[serde(rename = "monoisotopic-mass")]
    // pub monoisotopic_mass: f64,
    // pub name: String,
    // pub packagers: Packagers,
    // pub patents: Patents,
    // pub pathways: Pathways,
    // #[serde(rename = "pdb-entries")]
    // pub pdb_entries: PdbEntries,
    // pub pharmacodynamics: Pharmacodynamics,
    // pub prices: Prices,
    // pub products: Products,
    // #[serde(rename = "protein-binding")]
    // pub protein_binding: ProteinBinding,
    // pub reactions: Reactions,
    // #[serde(rename = "route-of-elimination")]
    // pub route_of_elimination: RouteOfElimination,
    // pub salts: Salts,
    // #[serde(rename = "snp-adverse-drug-reactions")]
    // pub snp_adverse_drug_reactions: SnpAdverseDrugReactions,
    // #[serde(rename = "snp-effects")]
    // pub snp_effects: SnpEffects,
    // pub synonyms: Synonyms,
    // #[serde(rename = "synthesis-reference")]
    // pub synthesis_reference: SynthesisReference,
    // pub targets: Targets,
    // pub toxicity: Toxicity,
    // pub transporters: Transporters,
    // pub unii: String,
    // #[serde(rename = "volume-of-distribution")]
    // pub volume_of_distribution: VolumeOfDistribution,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Absorption {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedOrganisms {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AhfsCodes {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtcCodes {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "property")]
    pub property: Option<PropertyEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyEnum {
    ARR(Vec<Property>),
    OO(Property),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub kind: String,
    pub source: ::serde_json::Value,
    pub value: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Carriers {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Categories {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clearance {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dosages {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugInteractions {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugbankId {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "@primary")]
    pub primary: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DrugbankIdEnum {
    Id(DrugbankId),
    Name(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enzymes {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIdentifiers {
    #[serde(rename = "external-identifier")]
    pub external_identifier: ExternalIdentifier,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIdentifier {
    pub identifier: i64,
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLinks {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodInteractions {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralReferences {
    pub articles: Articles,
    pub attachments: Attachments,
    pub links: Links,
    pub textbooks: Textbooks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Articles {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachments {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Textbooks {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Groups {
    pub group: serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HalfLife {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indication {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternationalBrands {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manufacturers {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MechanismOfAction {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metabolism {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mixtures {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Packagers {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patents {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pathways {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdbEntries {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pharmacodynamics {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prices {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Products {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProteinBinding {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactions {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteOfElimination {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Salts {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnpAdverseDrugReactions {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnpEffects {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonyms {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynthesisReference {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Targets {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toxicity {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transporters {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeOfDistribution {}

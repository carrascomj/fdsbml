use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Species {
    pub id: String,
    pub name: Option<String>,
    pub compartment: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub id: String,
    #[serde(default)]
    pub list_of_reactants: ListOfSpeciesReferences,
    #[serde(default)]
    pub list_of_products: ListOfSpeciesReferences,
    pub name: Option<String>,
    #[serde(rename = "fbc:lowerFluxBound")]
    pub lower_bound: Option<String>,
    #[serde(rename = "fbc:lowerUpperBound")]
    pub upper_bound: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SpeciesReference {
    pub species: String,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct ListOfSpeciesReferences {
    #[serde(rename = "speciesReference", default = "Vec::new")]
    pub species_references: Vec<SpeciesReference>,
}

#[derive(Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename = "model", rename_all = "camelCase")]
pub struct ModelRaw {
    #[serde(default)]
    pub list_of_species: ListOfSpecies,
    #[serde(default)]
    pub list_of_reactions: ListOfReactions,
}

impl ModelRaw {
    pub fn parse(doc: &str) -> Result<Self, quick_xml::DeError> {
        let raw_model: Sbml = quick_xml::de::from_str(doc)?;
        Ok(raw_model.model)
    }
}

#[derive(Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfSpecies {
    pub species: Vec<Species>,
}

#[derive(Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfReactions {
    #[serde(rename = "reaction", default)]
    pub reactions: Vec<Reaction>,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
struct Sbml {
    model: ModelRaw,
}

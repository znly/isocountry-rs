use std::{env, error::Error, path::Path};

use serde::{Deserialize, Serialize};
use tinytemplate::{format_unescaped, TinyTemplate};

/// A country as represented by the ISO 3166-1 standard
#[derive(Serialize, Deserialize)]
struct Country {
    alpha_2: String,
    alpha_3: String,
    name: String,
    numeric: u16,
}

/// A country subdivision as represented by the ISO 3166-2 standard
#[derive(Deserialize)]
struct Subdivision {
    code: String,
    name: String,
    #[serde(rename = "type")]
    ty: String,
    parent: Option<String>,
}

#[derive(Deserialize)]
struct SubdivisionDataset {
    #[serde(rename(deserialize = "3166-2"))]
    subdivisions: Vec<Subdivision>,
}

#[derive(Deserialize, Serialize)]
struct CountryDataset {
    #[serde(rename(deserialize = "3166-1"))]
    countries: Vec<Country>,
}

// -----------------------------------------------------------------------------

#[derive(Serialize)]
struct SubdivisionTemplate {
    code: String,
    code_identifier: String,
    name: String,
    #[serde(rename = "type")]
    ty: String,
    parent: Option<String>,
}

#[derive(Serialize)]
struct TemplateContext {
    subdivisions: Vec<SubdivisionTemplate>,
}

impl From<Subdivision> for SubdivisionTemplate {
    fn from(value: Subdivision) -> Self {
        Self {
            code_identifier: str::replace(value.code.as_str(), "-", "_"),
            code: value.code,
            name: value.name,
            ty: value.ty,
            parent: value.parent,
        }
    }
}

// -----------------------------------------------------------------------------

fn generate_subdivision_file(
    template: &str,
    values: impl Iterator<Item = Subdivision>,
) -> Result<String, Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("subdivision", template)?;
    tt.set_default_formatter(&format_unescaped);

    let ctx = TemplateContext {
        subdivisions: values.map(Into::into).collect(),
    };

    Ok(tt.render("subdivision", &ctx)?)
}

// -----------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn Error>> {
    let iso_3166_2_dataset = std::fs::read_to_string("build/data/data_iso_3166-2.json")?;
    let subdivision_dataset: SubdivisionDataset =
        serde_json::from_str(iso_3166_2_dataset.as_str())?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("subdivision.rs");

    let subdivision_template = std::fs::read_to_string("build/subdivision.template")?;

    std::fs::write(
        dest_path,
        generate_subdivision_file(
            subdivision_template.as_str(),
            subdivision_dataset.subdivisions.into_iter(),
        )
        .expect("failed to generate subdivisions"),
    )
    .unwrap();

    Ok(())
}

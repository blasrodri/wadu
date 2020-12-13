use std::convert::TryFrom;

use crate::error::WaduError;
use crate::gist::{identifier::GistIdentifier, Gist};

pub fn load_gist(identifier: &str) -> Result<Gist, WaduError> {
    let gist_identifier = GistIdentifier::try_from(identifier)?;
    get_gist(gist_identifier)
}

fn get_gist<'a>(gist_identifier: GistIdentifier<'a>) -> Result<Gist, WaduError> {
    let url = gist_identifier.get_url();
    let response = ureq::get(url.as_str())
        .set("Accept", "application/vnd.github.v3+json")
        .timeout_connect(1_000) // max 1 sec
        .build()
        .call();
    if response.error() {
        return Err(WaduError::GistError);
    }
    let json = response.into_json().unwrap();
    let files = json["files"].as_object();
    if files.is_none() {
        return Err(WaduError::GistError);
    }
    let files = files.unwrap();
    if !files.contains_key("Cargo.toml") || !files.contains_key("main.rs") {
        return Err(WaduError::GistError);
    }
    let cargo_file_obj = files.get("Cargo.toml").unwrap().as_object().unwrap();
    let main_rs_obj = files.get("main.rs").unwrap().as_object().unwrap();

    let cargo_url = url::Url::parse(cargo_file_obj["raw_url"].as_str().unwrap()).unwrap();
    let main_rs_url = url::Url::parse(main_rs_obj["raw_url"].as_str().unwrap()).unwrap();

    let cargo_file = fetch_file(&cargo_url)?;
    let main_rs_file = fetch_file(&main_rs_url)?;
    Ok(Gist {
        identifier: gist_identifier.get_identifier().to_string(),
        cargo: cargo_file,
        main_rs: main_rs_file,
    })
}

fn fetch_file(url: &url::Url) -> Result<String, WaduError> {
    let result = ureq::get(url.as_str()).build().call().into_string()?;
    Ok(result)
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_load_gist() {
        let gist_identifier = "d5e01931b83bf8396b3fa7c02f6a8db7";
        let result = load_gist(gist_identifier).unwrap();
        assert_eq!(
            Gist {
                identifier: gist_identifier.to_string(),
                main_rs: "//".to_string(),
                cargo: "#".to_string()
            },
            result
        );
    }
}

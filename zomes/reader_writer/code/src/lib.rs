#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use hdk::{
    error::ZomeApiError,
    holochain_core_types::hash::HashString,
    holochain_dna::zome::entry_types::Sharing,
};

use boolinator::*;

#[derive(Serialize, Deserialize)]
pub struct HoloText {
    holo_text: String,    
}

fn handle_holo_text_write(text: String) -> serde_json::Value {
    let maybe_address = hdk::commit_entry("holotext", json!({
        "holo_text": text,        
    }));
    match maybe_address {
        Ok(address) => {
            let link_result = hdk::link_entries(
                &HashString::from(hdk::AGENT_ADDRESS.to_string()),
                &address,
                "text"
            );

            if link_result.is_err() {
                return json!({"link error": link_result.err().unwrap()})
            }

            json!({"address": address})
        }
        Err(hdk_error) => hdk_error.to_json(),
    }
}

pub fn handle_holo_text_read() -> serde_json::Value {
    match hdk::get_links(&hdk::AGENT_ADDRESS, "text") {
        Ok(result) => {
            let mut holotexts: Vec<HoloText> = Vec::with_capacity(result.links.len());
            for address in result.links {
                let result : Result<Option<HoloText>, ZomeApiError> = hdk::get_entry(address);
                match result {
                    Ok(Some(holotext)) => holotexts.push(holotext),
                    Ok(None) =>  {},
                    Err(_) => {},
                }
            }
            json!(holotexts)
        },
        Err(hdk_error) => hdk_error.to_json(),
    }
}

define_zome! {
    entries: [
        entry!(
            name: "holotext",
            description: "",
            sharing: Sharing::Public,
            native_type: Post,

            validation_package: || {
                hdk::ValidationPackageDefinition::ChainFull
            },

            validation: |holotext: HoloText, _ctx: hdk::ValidationData| {
                (holotext.holo_text.len() < 280)
                    .ok_or_else(|| String::from("Text too long"))
            }
        )
    ]

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            
            text_write: {
                inputs: |holo_text: String|,
                outputs: |address: serde_json::Value|,
                handler: handle_holo_text_write
            }

            text_read: {
                inputs: | |,
                outputs: |holo_texts: serde_json::Value|,
                handler: handle_holo_text_read
            }
            
        }
    }
}

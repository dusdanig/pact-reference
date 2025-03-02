use ansi_term::Colour::*;

use pact_matching::Mismatch;
use pact_models::sync_interaction::RequestResponseInteraction;

use crate::{generate_display_for_result, MismatchResult};

pub fn process_request_response_result(
  interaction: &RequestResponseInteraction,
  match_result: &Result<Option<String>, MismatchResult>,
  output: &mut Vec<String>) {
  match match_result {
    Ok(_) => {
      generate_display_for_result(
        interaction.response.status,
        Green.paint("OK"),
        interaction.response.headers.clone().map(|h| h.iter().map(|(k, v)| {
          (k.clone(), v.join(", "), Green.paint("OK"))
        }).collect()), Green.paint("OK"),
        output
      );
    },
    Err(ref err) => match *err {
      MismatchResult::Error(ref err_des, _) => {
        output.push(format!("      {}", Red.paint(format!("Request Failed - {}", err_des))));
      },
      MismatchResult::Mismatches { ref mismatches, .. } => {
        let status_result = if mismatches.iter().any(|m| m.mismatch_type() == "StatusMismatch") {
          Red.paint("FAILED")
        } else {
          Green.paint("OK")
        };
        let header_results = match interaction.response.headers {
          Some(ref h) => Some(h.iter().map(|(k, v)| {
            (k.clone(), v.join(", "), if mismatches.iter().any(|m| {
              match *m {
                Mismatch::HeaderMismatch { ref key, .. } => k == key,
                _ => false
              }
            }) {
              Red.paint("FAILED")
            } else {
              Green.paint("OK")
            })
          }).collect()),
          None => None
        };
        let body_result = if mismatches.iter().any(|m| m.mismatch_type() == "BodyMismatch" ||
          m.mismatch_type() == "BodyTypeMismatch") {
          Red.paint("FAILED")
        } else {
          Green.paint("OK")
        };

        generate_display_for_result(interaction.response.status, status_result, header_results, body_result, output);
      }
    }
  }
}

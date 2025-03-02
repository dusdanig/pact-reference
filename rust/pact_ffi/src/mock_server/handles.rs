//! Handles wrapping Rust models

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::ptr::null_mut;
use std::str::from_utf8;
use std::sync::Mutex;

use anyhow::{anyhow, Context};
use bytes::Bytes;
use itertools::Itertools;
use lazy_static::*;
use libc::{c_char, c_uint, c_ushort, size_t};
use log::*;
use maplit::*;
use pact_models::{Consumer, PactSpecification, Provider};
use pact_models::bodies::OptionalBody;
use pact_models::content_types::ContentType;
use pact_models::generators::Generators;
use pact_models::http_parts::HttpPart;
use pact_models::interaction::Interaction;
use pact_models::json_utils::json_to_string;
use pact_models::matchingrules::{MatchingRule, MatchingRuleCategory, MatchingRules, RuleLogic};
use pact_models::pact::{ReadWritePact, write_pact};
use pact_models::path_exp::DocPath;
use pact_models::prelude::Pact;
use pact_models::prelude::v4::V4Pact;
use pact_models::provider_states::ProviderState;
use pact_models::v4::async_message::AsynchronousMessage;
use pact_models::v4::interaction::V4Interaction;
use pact_models::v4::sync_message::SynchronousMessage;
use pact_models::v4::synch_http::SynchronousHttp;
use serde_json::{json, Value};

use crate::{convert_cstr, ffi_fn, safe_str};
use crate::mock_server::{StringResult, xml};
use crate::mock_server::bodies::{
  empty_multipart_body,
  file_as_multipart_body,
  MultipartBody,
  process_json,
  process_object,
  request_multipart,
  response_multipart
};
use crate::models::iterators::{PactMessageIterator, PactSyncMessageIterator};
use crate::ptr;

#[derive(Debug, Clone)]
/// Pact handle inner struct
/// cbindgen:ignore
pub struct PactHandleInner {
  pub(crate) pact: V4Pact,
  pub(crate) mock_server_started: bool,
  pub(crate) specification_version: PactSpecification
}

lazy_static! {
  static ref PACT_HANDLES: Mutex<HashMap<u16, RefCell<PactHandleInner>>> = Mutex::new(hashmap![]);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// Wraps a Pact model struct
pub struct PactHandle {
  /// Pact reference
  pact_ref: u16
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// Wraps a Pact model struct
pub struct InteractionHandle {
  /// Interaction reference
  interaction_ref: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Request or Response enum
pub enum InteractionPart {
  /// Request part
  Request,
  /// Response part
  Response
}

impl PactHandle {
  /// Creates a new handle to a Pact model
  pub fn new(consumer: &str, provider: &str) -> Self {
    let mut handles = PACT_HANDLES.lock().unwrap();
    let id = (handles.len() + 1) as u16;
    let mut pact = V4Pact {
      consumer: Consumer { name: consumer.to_string() },
      provider: Provider { name: provider.to_string() },
      ..V4Pact::default()
    };
    pact.add_md_version("ffi", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
    handles.insert(id, RefCell::new(PactHandleInner {
      pact,
      mock_server_started: false,
      specification_version: PactSpecification::V3
    }));
    PactHandle {
      pact_ref: id
    }
  }

  /// Invokes the closure with the inner Pact model
  pub(crate) fn with_pact<R>(&self, f: &dyn Fn(u16, &mut PactHandleInner) -> R) -> Option<R> {
    let mut handles = PACT_HANDLES.lock().unwrap();
    handles.get_mut(&self.pact_ref).map(|inner| f(self.pact_ref - 1, &mut inner.borrow_mut()))
  }
}

impl InteractionHandle {
  /// Creates a new handle to an Interaction
  pub fn new(pact: PactHandle, interaction: u16) -> InteractionHandle {
    let mut index = pact.pact_ref as u32;
    index = index << 16;
    index = index + interaction as u32;
    InteractionHandle {
      interaction_ref: index
    }
  }

  /// Invokes the closure with the inner Pact model
  pub fn with_pact<R>(&self, f: &dyn Fn(u16, &mut PactHandleInner) -> R) -> Option<R> {
    let mut handles = PACT_HANDLES.lock().unwrap();
    let index = (self.interaction_ref >> 16) as u16;
    handles.get_mut(&index).map(|inner| f(index - 1, &mut inner.borrow_mut()))
  }

  /// Invokes the closure with the inner Interaction model
  pub fn with_interaction<R>(&self, f: &dyn Fn(u16, bool, &mut dyn V4Interaction) -> R) -> Option<R> {
    let mut handles = PACT_HANDLES.lock().unwrap();
    let index = (self.interaction_ref >> 16) as u16;
    let interaction = (self.interaction_ref & 0x0000FFFF) as u16;
    handles.get_mut(&index).map(|inner| {
      let inner_mut = &mut *inner.borrow_mut();
      let interactions = &mut inner_mut.pact.interactions;
      match interactions.get_mut((interaction - 1) as usize) {
        Some(inner_i) => {
          Some(f(interaction - 1, inner_mut.mock_server_started, inner_i.as_mut()))
        },
        None => None
      }
    }).flatten()
  }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// Wraps a Pact model struct
pub struct MessagePactHandle {
  /// Pact reference
  pact_ref: u16
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// Wraps a Pact model struct
pub struct MessageHandle {
  /// Interaction reference
  interaction_ref: u32
}

impl MessagePactHandle {
  /// Creates a new handle to a Pact model
  pub fn new(consumer: &str, provider: &str) -> Self {
    let mut handles = PACT_HANDLES.lock().unwrap();
    let id = (handles.len() + 1) as u16;
    let mut pact = V4Pact {
      consumer: Consumer { name: consumer.to_string() },
      provider: Provider { name: provider.to_string() },
      ..V4Pact::default()
    };
    pact.add_md_version("ffi", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
    handles.insert(id, RefCell::new(PactHandleInner {
      pact,
      mock_server_started: false,
      specification_version: PactSpecification::V3
    }));
    MessagePactHandle {
      pact_ref: id
    }
  }

  /// Invokes the closure with the inner model
  pub fn with_pact<R>(&self, f: &dyn Fn(u16, &mut V4Pact, PactSpecification) -> R) -> Option<R> {
    let mut handles = PACT_HANDLES.lock().unwrap();
    handles.get_mut(&self.pact_ref).map(|inner| {
      let mut ref_mut = inner.borrow_mut();
      let specification = ref_mut.specification_version;
      f(self.pact_ref - 1, &mut ref_mut.pact, specification)
    })
  }
}

impl MessageHandle {
  /// Creates a new handle to a message
  pub fn new(pact: MessagePactHandle, message: u16) -> MessageHandle {
    let mut index = pact.pact_ref as u32;
    index = index << 16;
    index = index + message as u32;
    MessageHandle {
      interaction_ref: index
    }
  }

  /// Creates a new handle to a message
  pub fn new_v4(pact: PactHandle, message: usize) -> MessageHandle {
    let mut index = pact.pact_ref as u32;
    index = index << 16;
    index = index + message as u32;
    MessageHandle {
      interaction_ref: index
    }
  }

  /// Invokes the closure with the inner model
  pub fn with_pact<R>(&self, f: &dyn Fn(u16, &mut V4Pact, PactSpecification) -> R) -> Option<R> {
    let mut handles = PACT_HANDLES.lock().unwrap();
    let index = self.interaction_ref as u16;
    handles.get_mut(&index).map(|inner| {
      let mut ref_mut = inner.borrow_mut();
      let specification = ref_mut.specification_version;
      f(index - 1, & mut ref_mut.pact, specification)
    })
  }

  /// Invokes the closure with the inner Interaction model
  pub fn with_message<R>(&self, f: &dyn Fn(u16, &mut dyn V4Interaction, PactSpecification) -> R) -> Option<R> {
    let mut handles = PACT_HANDLES.lock().unwrap();
    let index = (self.interaction_ref >> 16) as u16;
    let interaction = self.interaction_ref as u16;
    handles.get_mut(&index).map(|inner| {
      let mut ref_mut = inner.borrow_mut();
      let specification = ref_mut.specification_version;
      ref_mut.pact.interactions.get_mut((interaction - 1) as usize)
        .map(|inner_i| {
          if inner_i.is_message() {
            Some(f(interaction - 1, inner_i.as_mut(), specification))
          } else {
            error!("Interaction {:#x} is not a message interaction, it is {}", self.interaction_ref, inner_i.type_of());
            None
          }
        }).flatten()
    }).flatten()
  }
}


/// Creates a new Pact model and returns a handle to it.
///
/// * `consumer_name` - The name of the consumer for the pact.
/// * `provider_name` - The name of the provider for the pact.
///
/// Returns a new `PactHandle`. The handle will need to be freed with the `pactffi_free_pact_handle`
/// method to release its resources.
#[no_mangle]
pub extern fn pactffi_new_pact(consumer_name: *const c_char, provider_name: *const c_char) -> PactHandle {
  let consumer = convert_cstr("consumer_name", consumer_name).unwrap_or("Consumer");
  let provider = convert_cstr("provider_name", provider_name).unwrap_or("Provider");
  PactHandle::new(consumer, provider)
}

/// Creates a new HTTP Interaction and returns a handle to it.
///
/// * `description` - The interaction description. It needs to be unique for each interaction.
///
/// Returns a new `InteractionHandle`.
#[no_mangle]
pub extern fn pactffi_new_interaction(pact: PactHandle, description: *const c_char) -> InteractionHandle {
  if let Some(description) = convert_cstr("description", description) {
    pact.with_pact(&|_, inner| {
      let interaction = SynchronousHttp {
        description: description.to_string(),
        ..SynchronousHttp::default()
      };
      inner.pact.interactions.push(interaction.boxed_v4());
      InteractionHandle::new(pact, inner.pact.interactions.len() as u16)
    }).unwrap_or_else(|| InteractionHandle::new(pact, 0))
  } else {
    InteractionHandle::new(pact, 0)
  }
}

/// Creates a new message interaction and return a handle to it
/// * `description` - The interaction description. It needs to be unique for each interaction.
///
/// Returns a new `InteractionHandle`.
#[no_mangle]
pub extern fn pactffi_new_message_interaction(pact: PactHandle, description: *const c_char) -> InteractionHandle {
  if let Some(description) = convert_cstr("description", description) {
    pact.with_pact(&|_, inner| {
      let interaction = AsynchronousMessage {
        description: description.to_string(),
        ..AsynchronousMessage::default()
      };
      inner.pact.interactions.push(interaction.boxed_v4());
      InteractionHandle::new(pact, inner.pact.interactions.len() as u16)
    }).unwrap_or_else(|| InteractionHandle::new(pact, 0))
  } else {
    InteractionHandle::new(pact, 0)
  }
}

/// Creates a new synchronous message interaction (request/response) and return a handle to it
/// * `description` - The interaction description. It needs to be unique for each interaction.
///
/// Returns a new `InteractionHandle`.
#[no_mangle]
pub extern fn pactffi_new_sync_message_interaction(pact: PactHandle, description: *const c_char) -> InteractionHandle {
  if let Some(description) = convert_cstr("description", description) {
    pact.with_pact(&|_, inner| {
      let interaction = SynchronousMessage {
        description: description.to_string(),
        ..SynchronousMessage::default()
      };
      inner.pact.interactions.push(interaction.boxed_v4());
      InteractionHandle::new(pact, inner.pact.interactions.len() as u16)
    }).unwrap_or_else(|| InteractionHandle::new(pact, 0))
  } else {
    InteractionHandle::new(pact, 0)
  }
}

/// Sets the description for the Interaction. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `description` - The interaction description. It needs to be unique for each interaction.
#[no_mangle]
pub extern fn pactffi_upon_receiving(interaction: InteractionHandle, description: *const c_char) -> bool {
  if let Some(description) = convert_cstr("description", description) {
    interaction.with_interaction(&|_, mock_server_started, inner| {
      inner.set_description(description);
      !mock_server_started
    }).unwrap_or(false)
  } else {
    false
  }
}

/// Adds a provider state to the Interaction. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `description` - The provider state description. It needs to be unique.
#[no_mangle]
pub extern fn pactffi_given(interaction: InteractionHandle, description: *const c_char) -> bool {
  if let Some(description) = convert_cstr("description", description) {
    interaction.with_interaction(&|_, mock_server_started, inner| {
      inner.provider_states_mut().push(ProviderState::default(&description.to_string()));
      !mock_server_started
    }).unwrap_or(false)
  } else {
    false
  }
}

ffi_fn! {
    /// Sets the test name annotation for the interaction. This allows capturing the name of
    /// the test as metadata. This can only be used with V4 interactions.
    ///
    /// # Safety
    ///
    /// The test name parameter must be a valid pointer to a NULL terminated string.
    ///
    /// # Error Handling
    ///
    /// If the test name can not be set, this will return a positive value.
    ///
    /// * `1` - Function panicked. Error message will be available by calling `pactffi_get_error_message`.
    /// * `2` - Handle was not valid.
    /// * `3` - Mock server was already started and the integration can not be modified.
    /// * `4` - Not a V4 interaction.
    fn pactffi_interaction_test_name(interaction: InteractionHandle, test_name: *const c_char) -> c_uint {
      let test_name = safe_str!(test_name);
      interaction.with_interaction(&|_, started, inner| {
        if !started {
          if let Some(i) = inner.as_v4_mut() {
            i.comments_mut().insert("testname".to_string(), json!(test_name));
            0
          } else {
            4
          }
        } else {
          3
        }
      }).unwrap_or(2)
    } {
      1
    }
}

/// Adds a provider state to the Interaction with a parameter key and value. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `description` - The provider state description. It needs to be unique.
/// * `name` - Parameter name.
/// * `value` - Parameter value.
#[no_mangle]
pub extern fn pactffi_given_with_param(interaction: InteractionHandle, description: *const c_char,
                                       name: *const c_char, value: *const c_char) -> bool {
  if let Some(description) = convert_cstr("description", description) {
    if let Some(name) = convert_cstr("name", name) {
      let value = convert_cstr("value", value).unwrap_or_default();
      interaction.with_interaction(&|_, mock_server_started, inner| {
        let value = match serde_json::from_str(value) {
          Ok(json) => json,
          Err(_) => json!(value)
        };
        match inner.provider_states().iter().find_position(|state| state.name == description) {
          Some((index, _)) => {
            inner.provider_states_mut().get_mut(index).unwrap().params.insert(name.to_string(), value);
          },
          None => inner.provider_states_mut().push(ProviderState {
            name: description.to_string(),
            params: hashmap!{ name.to_string() => value }
          })
        };
        !mock_server_started
      }).unwrap_or(false)
    } else {
      false
    }
  } else {
    false
  }
}

/// Configures the request for the Interaction. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `method` - The request method. Defaults to GET.
/// * `path` - The request path. Defaults to `/`.
#[no_mangle]
pub extern fn pactffi_with_request(
  interaction: InteractionHandle,
  method: *const c_char,
  path: *const c_char
) -> bool {
  let method = convert_cstr("method", method).unwrap_or("GET");
  let path = convert_cstr("path", path).unwrap_or("/");

  interaction.with_interaction(&|_, mock_server_started, inner| {
    if let Some(reqres) = inner.as_v4_http_mut() {
      let path = from_integration_json(&mut reqres.request.matching_rules, &mut reqres.request.generators, &path.to_string(), DocPath::empty(), "path");
      reqres.request.method = method.to_string();
      reqres.request.path = path;
      !mock_server_started
    } else {
      error!("Interaction is not an HTTP interaction, is {}", inner.type_of());
      false
    }
  }).unwrap_or(false)
}

/// Configures a query parameter for the Interaction. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `name` - the query parameter name.
/// * `value` - the query parameter value.
/// * `index` - the index of the value (starts at 0). You can use this to create a query parameter with multiple values
#[no_mangle]
pub extern fn pactffi_with_query_parameter(
  interaction: InteractionHandle,
  name: *const c_char,
  index: size_t,
  value: *const c_char
) -> bool {
  if let Some(name) = convert_cstr("name", name) {
    let value = convert_cstr("value", value).unwrap_or_default();
    interaction.with_interaction(&|_, mock_server_started, inner| {
      if let Some(reqres) = inner.as_v4_http_mut() {
        reqres.request.query = reqres.request.query.clone().map(|mut q| {
          let mut path = DocPath::root();
          path.push_field(name).push_index(index);
          let value = from_integration_json(&mut reqres.request.matching_rules, &mut reqres.request.generators, &value.to_string(), path, "query");
          if q.contains_key(name) {
            let values = q.get_mut(name).unwrap();
            if index >= values.len() {
              values.resize_with(index + 1, Default::default);
            }
            values[index] = value;
          } else {
            let mut values: Vec<String> = Vec::new();
            values.resize_with(index + 1, Default::default);
            values[index] = value;
            q.insert(name.to_string(), values);
          };
          q
        }).or_else(|| {
          let mut path = DocPath::root();
          path.push_field(name).push_index(index);
          let value = from_integration_json(&mut reqres.request.matching_rules, &mut reqres.request.generators, &value.to_string(), path, "query");
          let mut values: Vec<String> = Vec::new();
          values.resize_with(index + 1, Default::default);
          values[index] = value;
          Some(hashmap! { name.to_string() => values })
        });
        !mock_server_started
      } else {
        error!("Interaction is not an HTTP interaction, is {}", inner.type_of());
        false
      }
    }).unwrap_or(false)
  } else {
    warn!("Ignoring query parameter with empty or null name");
    false
  }
}

/// Convert JSON matching rule structures into their internal representation (excl. bodies)
///
/// For non-body values (headers, query, path etc.) extract out the value from any matchers
/// and apply the matchers/generators to the model
fn from_integration_json(
  rules: &mut MatchingRules,
  generators: &mut Generators,
  value: &str,
  path: DocPath,
  category: &str,
) -> String {
  let category = rules.add_category(category);

  match serde_json::from_str(value) {
    Ok(json) => match json {
      serde_json::Value::Object(ref map) => {
        let json: serde_json::Value = process_object(map, category, generators, path, false, false);
        // These are simple JSON primitives (strings), so we must unescape them
        json_to_string(&json)
      },
      _ => value.to_string()
    },
    Err(_) => value.to_string()
  }
}

pub(crate) fn process_xml(body: String, matching_rules: &mut MatchingRuleCategory, generators: &mut Generators) -> Result<Vec<u8>, String> {
  trace!("process_xml");
  match serde_json::from_str(&body) {
    Ok(json) => match json {
      Value::Object(ref map) => xml::generate_xml_body(map, matching_rules, generators),
      _ => Err(format!("JSON document is invalid (expected an Object), have {}", json))
    },
    Err(err) => Err(format!("Failed to parse XML builder document: {}", err))
  }
}

/// Sets the specification version for a given Pact model. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started) or the version is invalid
///
/// * `pact` - Handle to a Pact model
/// * `version` - the spec version to use
#[no_mangle]
pub extern fn pactffi_with_specification(pact: PactHandle, version: PactSpecification) -> bool {
  pact.with_pact(&|_, inner| {
    inner.specification_version = version.into();
    !inner.mock_server_started
  }).unwrap_or(false)
}

/// Sets the additional metadata on the Pact file. Common uses are to add the client library details such as the name and version
/// Returns false if the interaction or Pact can't be modified (i.e. the mock server for it has already started)
///
/// * `pact` - Handle to a Pact model
/// * `namespace` - the top level metadat key to set any key values on
/// * `name` - the key to set
/// * `value` - the value to set
#[no_mangle]
pub extern fn pactffi_with_pact_metadata(
  pact: PactHandle,
  namespace: *const c_char,
  name: *const c_char,
  value: *const c_char
) -> bool {
  pact.with_pact(&|_, inner| {
    let namespace = convert_cstr("namespace", namespace).unwrap_or_default();
    let name = convert_cstr("name", name).unwrap_or_default();
    let value = convert_cstr("value", value).unwrap_or_default();

    if !namespace.is_empty() {
      inner.pact.metadata.insert(namespace.to_string(), json!({ name: value }));
    } else {
      warn!("no namespace provided for metadata {:?} => {:?}. Ignoring", name, value);
    }
    !inner.mock_server_started
  }).unwrap_or(false)
}

/// Configures a header for the Interaction. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `part` - The part of the interaction to add the header to (Request or Response).
/// * `name` - the header name.
/// * `value` - the header value.
/// * `index` - the index of the value (starts at 0). You can use this to create a header with multiple values
#[no_mangle]
pub extern fn pactffi_with_header(
  interaction: InteractionHandle,
  part: InteractionPart,
  name: *const c_char,
  index: size_t,
  value: *const c_char
) -> bool {
  if let Some(name) = convert_cstr("name", name) {
    let value = convert_cstr("value", value).unwrap_or_default();
    interaction.with_interaction(&|_, mock_server_started, inner| {
      if let Some(reqres) = inner.as_v4_http_mut() {
        let headers = match part {
          InteractionPart::Request => reqres.request.headers.clone(),
          InteractionPart::Response => reqres.response.headers.clone()
        };

        let mut path = DocPath::root();
        path.push_field(name);
        let value = match part {
          InteractionPart::Request => from_integration_json(
            &mut reqres.request.matching_rules,
            &mut reqres.request.generators,
            &value.to_string(),
            path,
            "header"),
          InteractionPart::Response => from_integration_json(
            &mut reqres.response.matching_rules,
            &mut reqres.response.generators,
            &value.to_string(),
            path,
            "header")
        };

        let updated_headers = headers.map(|mut h| {
          if h.contains_key(name) {
            let values = h.get_mut(name).unwrap();
            if index >= values.len() {
              values.resize_with(index + 1, Default::default);
            }
            values[index] = value.to_string();
          } else {
            let mut values: Vec<String> = Vec::new();
            values.resize_with(index + 1, Default::default);
            values[index] = value.to_string();
            h.insert(name.to_string(), values);
          };
          h
        }).or_else(|| {
          let mut values: Vec<String> = Vec::new();
          values.resize_with(index + 1, Default::default);
          values[index] = value.to_string();
          Some(hashmap! { name.to_string() => values })
        });
        match part {
          InteractionPart::Request => reqres.request.headers = updated_headers,
          InteractionPart::Response => reqres.response.headers = updated_headers
        };
        !mock_server_started
      } else {
        error!("Interaction is not an HTTP interaction, is {}", inner.type_of());
        false
      }
    }).unwrap_or(false)
  } else {
    warn!("Ignoring header with empty or null name");
    false
  }
}

/// Configures the response for the Interaction. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `status` - the response status. Defaults to 200.
#[no_mangle]
pub extern fn pactffi_response_status(interaction: InteractionHandle, status: c_ushort) -> bool {
  interaction.with_interaction(&|_, mock_server_started, inner| {
    if let Some(reqres) = inner.as_v4_http_mut() {
      reqres.response.status = status;
      !mock_server_started
    } else {
      error!("Interaction is not an HTTP interaction, is {}", inner.type_of());
      false
    }
  }).unwrap_or(false)
}

/// Adds the body for the interaction. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `part` - The part of the interaction to add the body to (Request or Response).
/// * `content_type` - The content type of the body. Defaults to `text/plain`. Will be ignored if a content type
///   header is already set.
/// * `body` - The body contents. For JSON payloads, matching rules can be embedded in the body.
#[no_mangle]
pub extern fn pactffi_with_body(
  interaction: InteractionHandle,
  part: InteractionPart,
  content_type: *const c_char,
  body: *const c_char
) -> bool {
  let content_type = convert_cstr("content_type", content_type).unwrap_or("text/plain");
  let body = convert_cstr("body", body).unwrap_or_default();
  let content_type_header = "Content-Type".to_string();
  interaction.with_interaction(&|_, mock_server_started, inner| {
    if let Some(reqres) = inner.as_v4_http_mut() {
      match part {
        InteractionPart::Request => {
          if !reqres.request.has_header(&content_type_header) {
            match reqres.request.headers {
              Some(ref mut headers) => {
                headers.insert(content_type_header.clone(), vec![content_type.to_string()]);
              },
              None => {
                reqres.request.headers = Some(hashmap! { content_type_header.clone() => vec![ content_type.to_string() ]});
              }
            }
          }
          let body = if reqres.request.content_type().unwrap_or_default().is_json() {
            let category = reqres.request.matching_rules.add_category("body");
            OptionalBody::Present(Bytes::from(process_json(body.to_string(), category, &mut reqres.request.generators)),
            Some("application/json".into()), None)
          } else if reqres.request.content_type().unwrap_or_default().is_xml() {
            let category = reqres.request.matching_rules.add_category("body");
            OptionalBody::Present(Bytes::from(process_xml(body.to_string(), category, &mut reqres.request.generators).unwrap_or(vec![])),
            Some("application/xml".into()), None)
          } else {
            OptionalBody::from(body)
          };
          reqres.request.body = body;
        },
        InteractionPart::Response => {
          if !reqres.response.has_header(&content_type_header) {
            match reqres.response.headers {
              Some(ref mut headers) => {
                headers.insert(content_type_header.clone(), vec![content_type.to_string()]);
              },
              None => {
                reqres.response.headers = Some(hashmap! { content_type_header.clone() => vec![ content_type.to_string() ]});
              }
            }
          }
          let body = if reqres.response.content_type().unwrap_or_default().is_json() {
            let category = reqres.response.matching_rules.add_category("body");
            OptionalBody::Present(Bytes::from(process_json(body.to_string(), category, &mut reqres.response.generators)),
            Some("application/json".into()), None)
          } else if reqres.response.content_type().unwrap_or_default().is_xml() {
            let category = reqres.response.matching_rules.add_category("body");
            OptionalBody::Present(Bytes::from(process_xml(body.to_string(), category, &mut reqres.response.generators).unwrap_or(vec![])),
            Some("application/xml".into()), None)
          } else {
            OptionalBody::from(body)
          };
          reqres.response.body = body;
        }
      };
      !mock_server_started
    } else {
      error!("Interaction is not an HTTP interaction, is {}", inner.type_of());
      false
    }
  }).unwrap_or(false)
}

/// Adds a binary file as the body with the expected content type and example contents. Will use
/// a mime type matcher to match the body. Returns false if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `interaction` - Interaction handle to set the body for.
/// * `part` - Request or response part.
/// * `content_type` - Expected content type.
/// * `body` - example body contents in bytes
/// * `size` - number of bytes in the body
#[no_mangle]
pub extern fn pactffi_with_binary_file(
  interaction: InteractionHandle,
  part: InteractionPart,
  content_type: *const c_char,
  body: *const u8 ,
  size: size_t
) -> bool {
  let content_type_header = "Content-Type".to_string();
  match convert_cstr("content_type", content_type) {
    Some(content_type) => {
      interaction.with_interaction(&|_, mock_server_started, inner| {
        if let Some(reqres) = inner.as_v4_http_mut() {
          match part {
            InteractionPart::Request => {
              reqres.request.body = convert_ptr_to_body(body, size);
              if !reqres.request.has_header(&content_type_header) {
                match reqres.request.headers {
                  Some(ref mut headers) => {
                    headers.insert(content_type_header.clone(), vec!["application/octet-stream".to_string()]);
                  },
                  None => {
                    reqres.request.headers = Some(hashmap! { content_type_header.clone() => vec!["application/octet-stream".to_string()]});
                  }
                }
              };
              reqres.request.matching_rules.add_category("body").add_rule(
                DocPath::root(), MatchingRule::ContentType(content_type.into()), RuleLogic::And);
            },
            InteractionPart::Response => {
              reqres.response.body = convert_ptr_to_body(body, size);
              if !reqres.response.has_header(&content_type_header) {
                match reqres.response.headers {
                  Some(ref mut headers) => {
                    headers.insert(content_type_header.clone(), vec!["application/octet-stream".to_string()]);
                  },
                  None => {
                    reqres.response.headers = Some(hashmap! { content_type_header.clone() => vec!["application/octet-stream".to_string()]});
                  }
                }
              }
              reqres.response.matching_rules.add_category("body").add_rule(
                DocPath::root(), MatchingRule::ContentType(content_type.into()), RuleLogic::And);
            }
          };
          !mock_server_started
        } else {
          error!("Interaction is not an HTTP interaction, is {}", inner.type_of());
          false
        }
      }).unwrap_or(false)
    },
    None => {
      warn!("with_binary_file: Content type value is not valid (NULL or non-UTF-8)");
      false
    }
  }
}

/// Adds a binary file as the body as a MIME multipart with the expected content type and example contents. Will use
/// a mime type matcher to match the body. Returns an error if the interaction or Pact can't be
/// modified (i.e. the mock server for it has already started)
///
/// * `interaction` - Interaction handle to set the body for.
/// * `part` - Request or response part.
/// * `content_type` - Expected content type of the file.
/// * `file` - path to the example file
/// * `part_name` - name for the mime part
#[no_mangle]
pub extern fn pactffi_with_multipart_file(
  interaction: InteractionHandle,
  part: InteractionPart,
  content_type: *const c_char,
  file: *const c_char,
  part_name: *const c_char
) -> StringResult {
  let part_name = convert_cstr("part_name", part_name).unwrap_or("file");
  match convert_cstr("content_type", content_type) {
    Some(content_type) => {
      match interaction.with_interaction(&|_, mock_server_started, inner| {
        match convert_ptr_to_mime_part_body(file, part_name) {
          Ok(body) => {
            if let Some(reqres) = inner.as_v4_http_mut() {
              match part {
                InteractionPart::Request => request_multipart(&mut reqres.request, &body.boundary, body.body, content_type, part_name),
                InteractionPart::Response => response_multipart(&mut reqres.response, &body.boundary, body.body, content_type, part_name)
              };
              if mock_server_started {
                Err("with_multipart_file: This Pact can not be modified, as the mock server has already started".to_string())
              } else {
                Ok(())
              }
            } else {
              error!("Interaction is not an HTTP interaction, is {}", inner.type_of());
              Err(format!("with_multipart_file: Interaction is not an HTTP interaction, is {}", inner.type_of()))
            }
          },
          Err(err) => Err(format!("with_multipart_file: failed to generate multipart body - {}", err))
        }
      }) {
        Some(result) => match result {
          Ok(_) => StringResult::Ok(null_mut()),
          Err(err) => {
            let error = CString::new(err).unwrap();
            StringResult::Failed(error.into_raw())
          }
        },
        None => {
          let error = CString::new("with_multipart_file: Interaction handle is invalid").unwrap();
          StringResult::Failed(error.into_raw())
        }
      }
    },
    None => {
      warn!("with_multipart_file: Content type value is not valid (NULL or non-UTF-8)");
      let error = CString::new("with_multipart_file: Content type value is not valid (NULL or non-UTF-8)").unwrap();
      StringResult::Failed(error.into_raw())
    }
  }
}

fn convert_ptr_to_body(body: *const u8, size: size_t) -> OptionalBody {
  if body.is_null() {
    OptionalBody::Null
  } else if size == 0 {
    OptionalBody::Empty
  } else {
    OptionalBody::Present(Bytes::from(unsafe { std::slice::from_raw_parts(body, size) }), None, None)
  }
}

fn convert_ptr_to_mime_part_body(file: *const c_char, part_name: &str) -> Result<MultipartBody, String> {
  if file.is_null() {
    empty_multipart_body()
  } else {
    let c_str = unsafe { CStr::from_ptr(file) };
    let file = match c_str.to_str() {
      Ok(str) => Ok(str),
      Err(err) => {
        warn!("convert_ptr_to_mime_part_body: Failed to parse file name as a UTF-8 string: {}", err);
        Err(format!("convert_ptr_to_mime_part_body: Failed to parse file name as a UTF-8 string: {}", err))
      }
    }?;
    file_as_multipart_body(file, part_name)
  }
}

ffi_fn! {
    /// Get an iterator over all the messages of the Pact. The returned iterator needs to be
    /// freed with `pactffi_pact_message_iter_delete`.
    ///
    /// # Safety
    ///
    /// The iterator contains a copy of the Pact, so it is always safe to use.
    ///
    /// # Error Handling
    ///
    /// On failure, this function will return a NULL pointer.
    ///
    /// This function may fail if any of the Rust strings contain embedded
    /// null ('\0') bytes.
    fn pactffi_pact_handle_get_message_iter(pact: PactHandle) -> *mut PactMessageIterator {
        let message_pact = pact.with_pact(&|_, inner| {
          // Ok to unwrap this, as the worse case given an HTTP Pact it will return a new message
          // pact with no messages
          inner.pact.as_message_pact().unwrap()
        }).ok_or_else(|| anyhow!("Pact handle is not valid"))?;
        let iter = PactMessageIterator::new(message_pact);
        ptr::raw_to(iter)
    } {
        ptr::null_mut_to::<PactMessageIterator>()
    }
}

ffi_fn! {
    /// Get an iterator over all the synchronous request/response messages of the Pact.
    /// The returned iterator needs to be freed with `pactffi_pact_sync_message_iter_delete`.
    ///
    /// # Safety
    ///
    /// The iterator contains a copy of the Pact, so it is always safe to use.
    ///
    /// # Error Handling
    ///
    /// On failure, this function will return a NULL pointer.
    ///
    /// This function may fail if any of the Rust strings contain embedded
    /// null ('\0') bytes.
    fn pactffi_pact_handle_get_sync_message_iter(pact: PactHandle) -> *mut PactSyncMessageIterator {
        let v4_pact = pact.with_pact(&|_, inner| {
          // Ok to unwrap this, as any non-v4 pact will be upgraded
          inner.pact.as_v4_pact().unwrap()
        }).ok_or_else(|| anyhow!("Pact handle is not valid"))?;
        let iter = PactSyncMessageIterator::new(v4_pact);
        ptr::raw_to(iter)
    } {
        ptr::null_mut_to::<PactSyncMessageIterator>()
    }
}

/// Creates a new Pact Message model and returns a handle to it.
///
/// * `consumer_name` - The name of the consumer for the pact.
/// * `provider_name` - The name of the provider for the pact.
///
/// Returns a new `MessagePactHandle`. The handle will need to be freed with the `pactffi_free_message_pact_handle`
/// function to release its resources.
#[no_mangle]
pub extern fn pactffi_new_message_pact(consumer_name: *const c_char, provider_name: *const c_char) -> MessagePactHandle {
  let consumer = convert_cstr("consumer_name", consumer_name).unwrap_or("Consumer");
  let provider = convert_cstr("provider_name", provider_name).unwrap_or("Provider");
  MessagePactHandle::new(consumer, provider)
}

/// Creates a new Message and returns a handle to it.
///
/// * `description` - The message description. It needs to be unique for each Message.
///
/// Returns a new `MessageHandle`.
#[no_mangle]
pub extern fn pactffi_new_message(pact: MessagePactHandle, description: *const c_char) -> MessageHandle {
  if let Some(description) = convert_cstr("description", description) {
    pact.with_pact(&|_, inner, _| {
      let message = AsynchronousMessage {
        description: description.to_string(),
        ..AsynchronousMessage::default()
      };
      inner.interactions.push(message.boxed_v4());
      MessageHandle::new(pact, inner.interactions.len() as u16)
    }).unwrap_or_else(|| MessageHandle::new(pact, 0))
  } else {
    MessageHandle::new(pact, 0)
  }
}

/// Sets the description for the Message.
///
/// * `description` - The message description. It needs to be unique for each message.
#[no_mangle]
pub extern fn pactffi_message_expects_to_receive(message: MessageHandle, description: *const c_char) {
  if let Some(description) = convert_cstr("description", description) {
    message.with_message(&|_, inner, _| {
      inner.set_description(description);
    });
  }
}

/// Adds a provider state to the Interaction.
///
/// * `description` - The provider state description. It needs to be unique for each message
#[no_mangle]
pub extern fn pactffi_message_given(message: MessageHandle, description: *const c_char) {
  if let Some(description) = convert_cstr("description", description) {
    message.with_message(&|_, inner, _| {
      inner.provider_states_mut().push(ProviderState::default(&description.to_string()));
    });
  }
}

/// Adds a provider state to the Message with a parameter key and value.
///
/// * `description` - The provider state description. It needs to be unique.
/// * `name` - Parameter name.
/// * `value` - Parameter value.
#[no_mangle]
pub extern fn pactffi_message_given_with_param(message: MessageHandle, description: *const c_char,
                                               name: *const c_char, value: *const c_char) {
  if let Some(description) = convert_cstr("description", description) {
    if let Some(name) = convert_cstr("name", name) {
      let value = convert_cstr("value", value).unwrap_or_default();
      message.with_message(&|_, inner, _| {
        let value = match serde_json::from_str(value) {
          Ok(json) => json,
          Err(_) => json!(value)
        };
        match inner.provider_states().iter().find_position(|state| state.name == description) {
          Some((index, _)) => {
            inner.provider_states_mut().get_mut(index).unwrap().params.insert(name.to_string(), value);
          },
          None => inner.provider_states_mut().push(ProviderState {
            name: description.to_string(),
            params: hashmap!{ name.to_string() => value }
          })
        };
      });
    }
  }
}

/// Adds the contents of the Message.
///
/// Accepts JSON, binary and other payload types. Binary data will be base64 encoded when serialised.
///
/// Note: For text bodies (plain text, JSON or XML), you can pass in a C string (NULL terminated)
/// and the size of the body is not required (it will be ignored). For binary bodies, you need to
/// specify the number of bytes in the body.
///
/// * `content_type` - The content type of the body. Defaults to `text/plain`, supports JSON structures with matchers and binary data.
/// * `body` - The body contents as bytes. For text payloads (JSON, XML, etc.), a C string can be used and matching rules can be embedded in the body.
/// * `content_type` - Expected content type (e.g. application/json, application/octet-stream)
/// * `size` - number of bytes in the message body to read. This is not required for text bodies (JSON, XML, etc.).
#[no_mangle]
pub extern fn pactffi_message_with_contents(message_handle: MessageHandle, content_type: *const c_char, body: *const u8, size: size_t) {
  let content_type = convert_cstr("content_type", content_type).unwrap_or("text/plain");
  trace!("pactffi_message_with_contents(message_handle: {:?}, content_type: {:?}, body: {:?}, size: {})", message_handle, content_type, body, size);

  message_handle.with_message(&|_, inner, _| {
    let content_type = ContentType::parse(content_type).ok();

    if let Some(message) = inner.as_v4_async_message_mut() {
      let body = if let Some(content_type) = content_type {
        let category = message.contents.matching_rules.add_category("body");
        let body_str = convert_cstr("body", body as *const c_char).unwrap_or_default();

        if content_type.is_xml() {
          OptionalBody::Present(Bytes::from(process_xml(body_str.to_string(), category, &mut message.contents.generators).unwrap_or(vec![])), Some(content_type), None)
        } else if content_type.is_text() || content_type.is_json() {
          OptionalBody::Present(Bytes::from(process_json(body_str.to_string(), category, &mut message.contents.generators)), Some(content_type), None)
        } else {
          OptionalBody::Present(Bytes::from(unsafe { std::slice::from_raw_parts(body, size) }), Some(content_type), None)
        }
      } else {
        OptionalBody::Present(Bytes::from(unsafe { std::slice::from_raw_parts(body, size) }), None, None)
      };

      message.contents.contents = body;
    }
  });
}

/// Adds expected metadata to the Message
///
/// * `key` - metadata key
/// * `value` - metadata value.
#[no_mangle]
pub extern fn pactffi_message_with_metadata(message_handle: MessageHandle, key: *const c_char, value: *const c_char) {
  if let Some(key) = convert_cstr("key", key) {
    let value = convert_cstr("value", value).unwrap_or_default();
    message_handle.with_message(&|_, inner, _| {
      if let Some(message) = inner.as_v4_async_message_mut() {
        message.contents.metadata.insert(key.to_string(), Value::String(value.to_string()));
      }
    });
  }
}

/// Reifies the given message
///
/// Reification is the process of stripping away any matchers, and returning the original contents.
/// NOTE: the returned string needs to be deallocated with the `free_string` function
#[no_mangle]
pub extern fn pactffi_message_reify(message_handle: MessageHandle) -> *const c_char {
  let res = message_handle.with_message(&|_, inner, spec_version| {
    trace!("pactffi_message_reify(message: {:?}, spec_version: {})", inner, spec_version);
    if let Some(message) = inner.as_v4_async_message() {
      match message.contents.contents {
        OptionalBody::Null => "null".to_string(),
        OptionalBody::Present(_, _, _) => if spec_version <= pact_models::PactSpecification::V3 {
          message.as_message().unwrap_or_default().to_json(&spec_version).to_string()
        } else {
          message.to_json().to_string()
        },
        _ => "".to_string()
      }
    } else {
      "".to_string()
    }
  });

  match res {
    Some(res) => {
      let string = CString::new(res).unwrap();
      string.into_raw() as *const c_char
    },
    None => CString::default().into_raw() as *const c_char
  }
}

/// External interface to write out the message pact file. This function should
/// be called if all the consumer tests have passed. The directory to write the file to is passed
/// as the second parameter. If a NULL pointer is passed, the current working directory is used.
///
/// If overwrite is true, the file will be overwritten with the contents of the current pact.
/// Otherwise, it will be merged with any existing pact file.
///
/// Returns 0 if the pact file was successfully written. Returns a positive code if the file can
/// not be written, or there is no mock server running on that port or the function panics.
///
/// # Errors
///
/// Errors are returned as positive values.
///
/// | Error | Description |
/// |-------|-------------|
/// | 1 | The pact file was not able to be written |
/// | 2 | The message pact for the given handle was not found |
#[no_mangle]
pub extern fn pactffi_write_message_pact_file(pact: MessagePactHandle, directory: *const c_char, overwrite: bool) -> i32 {
  let result = pact.with_pact(&|_, inner, spec_version| {
    let filename = path_from_dir(directory, Some(inner.default_file_name().as_str()));
    write_pact(inner.boxed(), &filename.unwrap(), spec_version, overwrite)
  });

  match result {
    Some(write_result) => match write_result {
      Ok(_) => 0,
      Err(e) => {
        log::error!("unable to write the pact file: {:}", e);
        1
      }
    },
    None => {
      log::error!("unable to write the pact file, message pact for handle {:?} not found", &pact);
      2
    }
  }
}

/// Sets the additional metadata on the Pact file. Common uses are to add the client library details such as the name and version
///
/// * `pact` - Handle to a Pact model
/// * `namespace` - the top level metadat key to set any key values on
/// * `name` - the key to set
/// * `value` - the value to set
#[no_mangle]
pub extern fn pactffi_with_message_pact_metadata(pact: MessagePactHandle, namespace: *const c_char, name: *const c_char, value: *const c_char) {
  pact.with_pact(&|_, inner, _| {
    let namespace = convert_cstr("namespace", namespace).unwrap_or_default();
    let name = convert_cstr("name", name).unwrap_or_default();
    let value = convert_cstr("value", value).unwrap_or_default();

    if !namespace.is_empty() {
      inner.metadata.insert(namespace.to_string(), json!({ name: value }));
    } else {
      warn!("no namespace provided for metadata {:?} => {:?}. Ignoring", name, value);
    }
  });
}

/// Given a c string for the output directory, and an optional filename
/// return a fully qualified directory or file path name for the output pact file
pub(crate) fn path_from_dir(directory: *const c_char, file_name: Option<&str>) -> Option<PathBuf> {
  let dir = unsafe {
    if directory.is_null() {
      log::warn!("Directory to write to is NULL, defaulting to the current working directory");
      None
    } else {
      let c_str = CStr::from_ptr(directory);
      let dir_str = from_utf8(c_str.to_bytes()).unwrap();
      if dir_str.is_empty() {
        None
      } else {
        Some(dir_str.to_string())
      }
    }
  };

  dir.map(|path| {
    let mut full_path = PathBuf::from(path);
    if let Some(pact_file_name) = file_name {
      full_path.push(pact_file_name);
    }
    full_path
  })
}

ffi_fn! {
  /// External interface to write out the pact file. This function should
  /// be called if all the consumer tests have passed. The directory to write the file to is passed
  /// as the second parameter. If a NULL pointer is passed, the current working directory is used.
  ///
  /// If overwrite is true, the file will be overwritten with the contents of the current pact.
  /// Otherwise, it will be merged with any existing pact file.
  ///
  /// Returns 0 if the pact file was successfully written. Returns a positive code if the file can
  /// not be written or the function panics.
  ///
  /// # Safety
  ///
  /// The directory parameter must either be NULL or point to a valid NULL terminated string.
  ///
  /// # Errors
  ///
  /// Errors are returned as positive values.
  ///
  /// | Error | Description |
  /// |-------|-------------|
  /// | 1 | The function panicked. |
  /// | 2 | The pact file was not able to be written. |
  /// | 3 | The pact for the given handle was not found. |
  fn pactffi_pact_handle_write_file(pact: PactHandle, directory: *const c_char, overwrite: bool) -> i32 {
    let result = pact.with_pact(&|_, inner| {
      let pact_file = inner.pact.default_file_name();
      let filename = path_from_dir(directory, Some(pact_file.as_str()));
      write_pact(inner.pact.boxed(), &filename.unwrap_or_else(|| PathBuf::from(pact_file.as_str())), inner.specification_version, overwrite)
    });

    match result {
      Some(write_result) => match write_result {
        Ok(_) => 0,
        Err(e) => {
          log::error!("unable to write the pact file: {:}", e);
          2
        }
      },
      None => {
        log::error!("unable to write the pact file, message pact for handle {:?} not found", &pact);
        3
      }
    }
  } {
    1
  }
}

/// Creates a new V4 asynchronous message and returns a handle to it.
///
/// * `description` - The message description. It needs to be unique for each Message.
///
/// Returns a new `MessageHandle`.
#[no_mangle]
pub extern fn pactffi_new_async_message(pact: PactHandle, description: *const c_char) -> MessageHandle {
  if let Some(description) = convert_cstr("description", description) {
    pact.with_pact(&|_, inner| {
      let message = AsynchronousMessage {
        description: description.to_string(),
        ..AsynchronousMessage::default()
      };
      inner.pact.interactions.push(message.boxed_v4());
      MessageHandle::new_v4(pact, inner.pact.interactions.len())
    }).unwrap_or_else(|| MessageHandle::new_v4(pact, 0))
  } else {
    MessageHandle::new_v4(pact, 0)
  }
}

/// Delete a Pact handle and free the resources used by it.
///
/// # Error Handling
///
/// On failure, this function will return a positive integer value.
///
/// * `1` - The handle is not valid or does not refer to a valid Pact. Could be that it was previously deleted.
///
#[no_mangle]
pub extern fn pactffi_free_pact_handle(pact: PactHandle) -> c_uint {
  let mut handles = PACT_HANDLES.lock().unwrap();
  handles.remove(&pact.pact_ref).map(|_| 0).unwrap_or(1)
}

/// Delete a Pact handle and free the resources used by it.
///
/// # Error Handling
///
/// On failure, this function will return a positive integer value.
///
/// * `1` - The handle is not valid or does not refer to a valid Pact. Could be that it was previously deleted.
///
#[no_mangle]
pub extern fn pactffi_free_message_pact_handle(pact: MessagePactHandle) -> c_uint {
  let mut handles = PACT_HANDLES.lock().unwrap();
  handles.remove(&pact.pact_ref).map(|_| 0).unwrap_or(1)
}

#[cfg(test)]
mod tests {
  use std::ffi::CString;

  use expectest::prelude::*;

  use crate::mock_server::handles::{
    pactffi_free_pact_handle,
    pactffi_new_async_message,
    pactffi_new_interaction,
    PactHandle
  };

  #[test]
  fn pact_handles() {
    let pact_handle = PactHandle::new("TestC", "TestP");
    let description = CString::new("first interaction").unwrap();
    let i_handle = pactffi_new_interaction(pact_handle, description.as_ptr());

    let description2 = CString::new("second interaction").unwrap();
    let i_handle2 = pactffi_new_async_message(pact_handle, description2.as_ptr());

    expect!(i_handle.interaction_ref).to(be_equal_to(((pact_handle.pact_ref as u32) << 16) + 1));
    expect!(i_handle2.interaction_ref).to(be_equal_to(((pact_handle.pact_ref as u32) << 16) + 2));

    pact_handle.with_pact(&|pact_ref, inner| {
      expect!(pact_ref).to(be_equal_to(pact_handle.pact_ref - 1));
      expect!(inner.pact.consumer.name.as_str()).to(be_equal_to("TestC"));
      expect!(inner.pact.provider.name.as_str()).to(be_equal_to("TestP"));
      expect!(inner.pact.interactions.len()).to(be_equal_to(2));
    });

    i_handle.with_interaction(&|i_ref, _, inner| {
      expect!(i_ref).to(be_equal_to(0));
      expect!(inner.description().as_str()).to(be_equal_to("first interaction"));
      expect!(inner.type_of().as_str()).to(be_equal_to("V4 Synchronous/HTTP"));
    });

    i_handle2.with_message(&|i_ref, inner, _| {
      expect!(i_ref).to(be_equal_to(1));
      expect!(inner.description().as_str()).to(be_equal_to("second interaction"));
      expect!(inner.type_of().as_str()).to(be_equal_to("V4 Asynchronous/Messages"));
    });

    pactffi_free_pact_handle(pact_handle);
  }
}

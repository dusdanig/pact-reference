use std::ffi::{CStr, CString};

use bytes::Bytes;
use expectest::prelude::*;
use libc::c_char;
use maplit::*;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;

use pact_ffi::mock_server::{
  pactffi_cleanup_mock_server,
  pactffi_create_mock_server,
  pactffi_create_mock_server_for_pact,
  pactffi_mock_server_mismatches,
  pactffi_write_pact_file
};
use pact_ffi::mock_server::handles::{
  InteractionPart,
  pactffi_message_expects_to_receive,
  pactffi_message_given,
  pactffi_message_reify,
  pactffi_message_with_contents,
  pactffi_message_with_metadata,
  pactffi_new_interaction,
  pactffi_new_message,
  pactffi_new_message_pact,
  pactffi_new_pact,
  pactffi_response_status,
  pactffi_upon_receiving,
  pactffi_with_body,
  pactffi_with_header,
  pactffi_with_multipart_file,
  pactffi_with_query_parameter,
  pactffi_with_request,
  pactffi_write_message_pact_file
};
use pact_ffi::verifier::{OptionsFlags, pactffi_verifier_cli_args};
use pact_models::bodies::OptionalBody;

#[test]
fn post_to_mock_server_with_mismatches() {
  let pact_json = include_str!("post-pact.json");
  let pact_json_c = CString::new(pact_json).expect("Could not construct C string from json");
  let address = CString::new("127.0.0.1:0").unwrap();
  #[allow(deprecated)]
  let port = pactffi_create_mock_server(pact_json_c.as_ptr(), address.as_ptr(), false);
  expect!(port).to(be_greater_than(0));

  let client = Client::default();
  client.post(format!("http://127.0.0.1:{}/path", port).as_str())
    .header(CONTENT_TYPE, "application/json")
    .body(r#"{"foo":"no-very-bar"}"#)
    .send().expect("Sent POST request to mock server");

  let mismatches = unsafe {
    CStr::from_ptr(pactffi_mock_server_mismatches(port)).to_string_lossy().into_owned()
  };

  pactffi_cleanup_mock_server(port);

  expect!(mismatches).to(be_equal_to("[{\"method\":\"POST\",\"mismatches\":[{\"actual\":\"\\\"no-very-bar\\\"\",\"expected\":\"\\\"bar\\\"\",\"mismatch\":\"Expected \'bar\' to be equal to \'no-very-bar\'\",\"path\":\"$.foo\",\"type\":\"BodyMismatch\"}],\"path\":\"/path\",\"type\":\"request-mismatch\"}]"));
}

#[test]
fn create_header_with_multiple_values() {
  let consumer_name = CString::new("consumer").unwrap();
  let provider_name = CString::new("provider").unwrap();
  let pact_handle = pactffi_new_pact(consumer_name.as_ptr(), provider_name.as_ptr());
  let description = CString::new("create_header_with_multiple_values").unwrap();
  let interaction = pactffi_new_interaction(pact_handle, description.as_ptr());
  let name = CString::new("accept").unwrap();
  let value_1 = CString::new("application/hal+json").unwrap();
  let value_2 = CString::new("application/json").unwrap();
  pactffi_with_header(interaction.clone(), InteractionPart::Request, name.as_ptr(), 1, value_2.as_ptr());
  pactffi_with_header(interaction.clone(), InteractionPart::Request, name.as_ptr(), 0, value_1.as_ptr());
  interaction.with_interaction(&|_, _, i| {
    let interaction = i.as_v4_http().unwrap();
    expect!(interaction.request.headers.as_ref()).to(be_some().value(&hashmap!{
      "accept".to_string() => vec!["application/hal+json".to_string(), "application/json".to_string()]
    }));
  });
}

#[test]
fn create_query_parameter_with_multiple_values() {
  let consumer_name = CString::new("consumer").unwrap();
  let provider_name = CString::new("provider").unwrap();
  let pact_handle = pactffi_new_pact(consumer_name.as_ptr(), provider_name.as_ptr());
  let description = CString::new("create_query_parameter_with_multiple_values").unwrap();
  let interaction = pactffi_new_interaction(pact_handle, description.as_ptr());
  let name = CString::new("q").unwrap();
  let value_1 = CString::new("1").unwrap();
  let value_2 = CString::new("2").unwrap();
  let value_3 = CString::new("3").unwrap();
  pactffi_with_query_parameter(interaction.clone(), name.as_ptr(), 2, value_3.as_ptr());
  pactffi_with_query_parameter(interaction.clone(), name.as_ptr(), 0, value_1.as_ptr());
  pactffi_with_query_parameter(interaction.clone(), name.as_ptr(), 1, value_2.as_ptr());
  interaction.with_interaction(&|_, _, i| {
    let interaction = i.as_v4_http().unwrap();
    expect!(interaction.request.query.as_ref()).to(be_some().value(&hashmap!{
      "q".to_string() => vec!["1".to_string(), "2".to_string(), "3".to_string()]
    }));
  });
}

#[test]
fn create_multipart_file() {
  let consumer_name = CString::new("consumer").unwrap();
  let provider_name = CString::new("provider").unwrap();
  let pact_handle = pactffi_new_pact(consumer_name.as_ptr(), provider_name.as_ptr());
  let description = CString::new("create_multipart_file").unwrap();
  let interaction = pactffi_new_interaction(pact_handle, description.as_ptr());
  let content_type = CString::new("application/json").unwrap();
  let file = CString::new("tests/multipart-test-file.json").unwrap();
  let part_name = CString::new("file").unwrap();

  pactffi_with_multipart_file(interaction.clone(), InteractionPart::Request, content_type.as_ptr(), file.as_ptr(), part_name.as_ptr());

  interaction.with_interaction(&|_, _, i| {
    let interaction = i.as_v4_http().unwrap();
    let boundary = match &interaction.request.headers {
      Some(hashmap) => {
        hashmap.get("Content-Type")
          .map(|vec| vec[0].as_str())
          // Sorry for awful mime parsing..
          .map(|content_type: &str| content_type.split("boundary=").collect::<Vec<_>>())
          .map(|split| split[1])
          .unwrap_or("")
      },
      None => ""
    };

    expect!(interaction.request.headers.as_ref()).to(be_some().value(&hashmap!{
      "Content-Type".to_string() => vec![format!("multipart/form-data; boundary={}", boundary)],
    }));

    let actual_req_body_str = match &interaction.request.body {
      OptionalBody::Present(body, _, _) => body.clone(),
      _ => Bytes::new(),
    };

    let expected_req_body = Bytes::from(format!(
      "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"multipart-test-file.json\"\r\nContent-Type: application/json\r\n\r\ntrue\r\n--{boundary}--\r\n",
      boundary = boundary
    ));

    expect!(actual_req_body_str).to(be_equal_to(expected_req_body));
  });
}

#[test_log::test]
fn http_consumer_feature_test() {
  let consumer_name = CString::new("http-consumer").unwrap();
  let provider_name = CString::new("http-provider").unwrap();
  let pact_handle = pactffi_new_pact(consumer_name.as_ptr(), provider_name.as_ptr());
  let description = CString::new("request_with_matchers").unwrap();
  let interaction = pactffi_new_interaction(pact_handle.clone(), description.as_ptr());
  let special_header = CString::new("My-Special-Content-Type").unwrap();
  let content_type = CString::new("Content-Type").unwrap();
  let authorization = CString::new("Authorization").unwrap();
  let path_matcher = CString::new("{\"value\":\"/request/1234\",\"pact:matcher:type\":\"regex\", \"regex\":\"\\/request\\/[0-9]+\"}").unwrap();
  let value_header_with_matcher = CString::new("{\"value\":\"application/json\",\"pact:matcher:type\":\"dummy\"}").unwrap();
  let auth_header_with_matcher = CString::new("{\"value\":\"Bearer 1234\",\"pact:matcher:type\":\"regex\", \"regex\":\"Bearer [0-9]+\"}").unwrap();
  let query_param_matcher = CString::new("{\"value\":\"bar\",\"pact:matcher:type\":\"regex\", \"regex\":\"(bar|baz|bat)\"}").unwrap();
  let request_body_with_matchers = CString::new("{\"id\": {\"value\":1,\"pact:matcher:type\":\"type\"}}").unwrap();
  let response_body_with_matchers = CString::new("{\"created\": {\"value\":\"maybe\",\"pact:matcher:type\":\"regex\", \"regex\":\"(yes|no|maybe)\"}}").unwrap();
  let address = CString::new("127.0.0.1:0").unwrap();
  let file_path = CString::new("/tmp/pact").unwrap();
  let description = CString::new("a request to test the FFI interface").unwrap();
  let method = CString::new("POST").unwrap();
  let query =  CString::new("foo").unwrap();
  let header = CString::new("application/json").unwrap();

  pactffi_upon_receiving(interaction.clone(), description.as_ptr());
  pactffi_with_request(interaction.clone(), method  .as_ptr(), path_matcher.as_ptr());
  pactffi_with_header(interaction.clone(), InteractionPart::Request, content_type.as_ptr(), 0, value_header_with_matcher.as_ptr());
  pactffi_with_header(interaction.clone(), InteractionPart::Request, authorization.as_ptr(), 0, auth_header_with_matcher.as_ptr());
  pactffi_with_query_parameter(interaction.clone(), query.as_ptr(), 0, query_param_matcher.as_ptr());
  pactffi_with_body(interaction.clone(), InteractionPart::Request, header.as_ptr(), request_body_with_matchers.as_ptr());
  // will respond with...
  pactffi_with_header(interaction.clone(), InteractionPart::Response, content_type.as_ptr(), 0, value_header_with_matcher.as_ptr());
  pactffi_with_header(interaction.clone(), InteractionPart::Response, special_header.as_ptr(), 0, value_header_with_matcher.as_ptr());
  pactffi_with_body(interaction.clone(), InteractionPart::Response, header.as_ptr(), response_body_with_matchers.as_ptr());
  pactffi_response_status(interaction.clone(), 200);
  let port = pactffi_create_mock_server_for_pact(pact_handle.clone(), address.as_ptr(), false);

  expect!(port).to(be_greater_than(0));

  // Mock server has started, we can't now modify the pact
  expect!(pactffi_upon_receiving(interaction.clone(), description.as_ptr())).to(be_false());

  let client = Client::default();
  let result = client.post(format!("http://127.0.0.1:{}/request/9999?foo=baz", port).as_str())
    .header("Content-Type", "application/json")
    .header("Authorization", "Bearer 9999")
    .body(r#"{"id": 7}"#)
    .send();

  match result {
    Ok(res) => {
      dbg!("here");
      expect!(res.status()).to(be_eq(200));
      expect!(res.headers().get("My-Special-Content-Type").unwrap()).to(be_eq("application/json"));
      let json: serde_json::Value = res.json().unwrap_or_default();
      dbg!("about to check");
      expect!(json.get("created").unwrap().as_str().unwrap()).to(be_eq("maybe"));
    },
    Err(_) => {
      panic!("expected 200 response but request failed");
    }
  };

  let mismatches = unsafe {
    CStr::from_ptr(pactffi_mock_server_mismatches(port)).to_string_lossy().into_owned()
  };

  pactffi_write_pact_file(port, file_path.as_ptr(), true);
  pactffi_cleanup_mock_server(port);

  expect!(mismatches).to(be_equal_to("[]"));
}

#[test]
fn http_xml_consumer_feature_test() {
  let consumer_name = CString::new("http-consumer").unwrap();
  let provider_name = CString::new("http-provider").unwrap();
  let pact_handle = pactffi_new_pact(consumer_name.as_ptr(), provider_name.as_ptr());
  let description = CString::new("request_with_matchers").unwrap();
  let interaction = pactffi_new_interaction(pact_handle.clone(), description.as_ptr());
  let accept = CString::new("Accept").unwrap();
  let content_type = CString::new("Content-Type").unwrap();
  let response_body_with_matchers = CString::new(r#"{"version":"1.0","charset":"UTF-8","root":{"name":"ns1:projects","children":[{"pact:matcher:type":"type","value":{"name":"ns1:project","children":[{"name":"ns1:tasks","children":[{"pact:matcher:type":"type","value":{"name":"ns1:task","children":[],"attributes":{"id":{"pact:matcher:type":"integer","value":1},"name":{"pact:matcher:type":"type","value":"Task 1"},"done":{"pact:matcher:type":"type","value":true}}},"examples":5}],"attributes":{}}],"attributes":{"id":{"pact:matcher:type":"integer","value":1},"type":"activity","name":{"pact:matcher:type":"type","value":"Project 1"}}},"examples":2}],"attributes":{"id":"1234","xmlns:ns1":"http://some.namespace/and/more/stuff"}}}"#).unwrap();
  let address = CString::new("127.0.0.1:0").unwrap();
  let file_path = CString::new("/tmp/pact").unwrap();
  let description = CString::new("a request to test the FFI interface").unwrap();
  let method = CString::new("GET").unwrap();
  let path = CString::new("/xml").unwrap();
  let header = CString::new("application/xml").unwrap();

  pactffi_upon_receiving(interaction.clone(), description.as_ptr());
  pactffi_with_request(interaction.clone(), method.as_ptr(), path.as_ptr());
  pactffi_with_header(interaction.clone(), InteractionPart::Request, accept.as_ptr(), 0, header.as_ptr());
  // will respond with...
  pactffi_with_header(interaction.clone(), InteractionPart::Response, content_type.as_ptr(), 0, header.as_ptr());
  pactffi_with_body(interaction.clone(), InteractionPart::Response, header.as_ptr(), response_body_with_matchers.as_ptr());
  pactffi_response_status(interaction.clone(), 200);
  let port = pactffi_create_mock_server_for_pact(pact_handle.clone(), address.as_ptr(), false);

  expect!(port).to(be_greater_than(0));

  // Mock server has started, we can't now modify the pact
  expect!(pactffi_upon_receiving(interaction.clone(), description.as_ptr())).to(be_false());

  let client = Client::default();
  let result = client.get(format!("http://127.0.0.1:{}/xml", port).as_str())
    .header("Accept", "application/xml")
    .send();

  match result {
    Ok(res) => {
      expect!(res.status()).to(be_eq(200));
      expect!(res.headers().get("Content-Type").unwrap()).to(be_eq("application/xml"));
      expect!(res.text().unwrap_or_default()).to(be_equal_to("<?xml version='1.0'?><ns1:projects id='1234' xmlns:ns1='http://some.namespace/and/more/stuff'><ns1:project id='1' name='Project 1' type='activity'><ns1:tasks><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/></ns1:tasks></ns1:project><ns1:project id='1' name='Project 1' type='activity'><ns1:tasks><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/></ns1:tasks></ns1:project></ns1:projects>"));
    },
    Err(_) => {
      panic!("expected 200 response but request failed");
    }
  };

  let mismatches = unsafe {
    CStr::from_ptr(pactffi_mock_server_mismatches(port)).to_string_lossy().into_owned()
  };

  pactffi_write_pact_file(port, file_path.as_ptr(), true);
  pactffi_cleanup_mock_server(port);

  expect!(mismatches).to(be_equal_to("[]"));
}

#[test]
fn message_consumer_feature_test() {
  let consumer_name = CString::new("message-consumer").unwrap();
  let provider_name = CString::new("message-provider").unwrap();
  let description = CString::new("message_request_with_matchers").unwrap();
  let content_type = CString::new("application/json").unwrap();
  let metadata_key = CString::new("message-queue-name").unwrap();
  let metadata_val = CString::new("message-queue-val").unwrap();
  let request_body_with_matchers = CString::new("{\"id\": {\"value\":1,\"pact:matcher:type\":\"type\"}}").unwrap();
  let file_path = CString::new("/tmp/pact").unwrap();
  let given = CString::new("a functioning FFI interface").unwrap();
  let receive_description = CString::new("a request to test the FFI interface").unwrap();

  let message_pact_handle = pactffi_new_message_pact(consumer_name.as_ptr(), provider_name.as_ptr());
  let message_handle = pactffi_new_message(message_pact_handle.clone(), description.as_ptr());
  pactffi_message_given(message_handle.clone(), given.as_ptr());
  pactffi_message_expects_to_receive(message_handle.clone(), receive_description.as_ptr());
  let body_bytes = request_body_with_matchers.as_bytes();
  pactffi_message_with_contents(message_handle.clone(), content_type.as_ptr(), body_bytes.as_ptr(), body_bytes.len());
  pactffi_message_with_metadata(message_handle.clone(), metadata_key.as_ptr(), metadata_val.as_ptr());
  let res: *const c_char = pactffi_message_reify(message_handle.clone());
  let reified: &CStr = unsafe { CStr::from_ptr(res) };
  expect!(reified.to_str().to_owned()).to(be_ok().value("{\"contents\":{\"id\":1},\"description\":\"a request to test the FFI interface\",\"matchingRules\":{\"body\":{\"$.id\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"type\"}]}}},\"metadata\":{\"contentType\":\"application/json\",\"message-queue-name\":\"message-queue-val\"},\"providerStates\":[{\"name\":\"a functioning FFI interface\"}]}".to_string()));
  let res = pactffi_write_message_pact_file(message_pact_handle.clone(), file_path.as_ptr(), true);
  expect!(res).to(be_eq(0));
}

#[test]
fn message_xml_consumer_feature_test() {
  let consumer_name = CString::new("message-consumer").unwrap();
  let provider_name = CString::new("message-provider").unwrap();
  let description = CString::new("message_request_with_matchers").unwrap();
  let content_type = CString::new("application/xml").unwrap();
  let metadata_key = CString::new("message-queue-name").unwrap();
  let metadata_val = CString::new("message-queue-val").unwrap();
  let request_body_with_matchers = CString::new(r#"{"version":"1.0","charset":"UTF-8","root":{"name":"ns1:projects","children":[{"pact:matcher:type":"type","value":{"name":"ns1:project","children":[{"name":"ns1:tasks","children":[{"pact:matcher:type":"type","value":{"name":"ns1:task","children":[],"attributes":{"id":{"pact:matcher:type":"integer","value":1},"name":{"pact:matcher:type":"type","value":"Task 1"},"done":{"pact:matcher:type":"type","value":true}}},"examples":5}],"attributes":{}}],"attributes":{"id":{"pact:matcher:type":"integer","value":1},"type":"activity","name":{"pact:matcher:type":"type","value":"Project 1"}}},"examples":2}],"attributes":{"id":"1234","xmlns:ns1":"http://some.namespace/and/more/stuff"}}}"#).unwrap();
  let file_path = CString::new("/tmp/pact").unwrap();
  let given = CString::new("a functioning FFI interface").unwrap();
  let receive_description = CString::new("a request to test the FFI interface").unwrap();

  let message_pact_handle = pactffi_new_message_pact(consumer_name.as_ptr(), provider_name.as_ptr());
  let message_handle = pactffi_new_message(message_pact_handle.clone(), description.as_ptr());
  pactffi_message_given(message_handle.clone(), given.as_ptr());
  pactffi_message_expects_to_receive(message_handle.clone(), receive_description.as_ptr());
  let body_bytes = request_body_with_matchers.as_bytes();
  pactffi_message_with_contents(message_handle.clone(), content_type.as_ptr(), body_bytes.as_ptr(), body_bytes.len());
  pactffi_message_with_metadata(message_handle.clone(), metadata_key.as_ptr(), metadata_val.as_ptr());
  let res: *const c_char = pactffi_message_reify(message_handle.clone());
  let reified: &CStr = unsafe { CStr::from_ptr(res) };
  expect!(reified.to_str().to_owned()).to(be_ok().value("{\"contents\":\"<?xml version='1.0'?><ns1:projects id='1234' xmlns:ns1='http://some.namespace/and/more/stuff'><ns1:project id='1' name='Project 1' type='activity'><ns1:tasks><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/></ns1:tasks></ns1:project><ns1:project id='1' name='Project 1' type='activity'><ns1:tasks><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/><ns1:task done='true' id='1' name='Task 1'/></ns1:tasks></ns1:project></ns1:projects>\",\"description\":\"a request to test the FFI interface\",\"matchingRules\":{\"body\":{\"$.ns1:projects.ns1:project\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"type\"}]},\"$.ns1:projects.ns1:project.ns1:tasks.ns1:task\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"type\"}]},\"$.ns1:projects.ns1:project.ns1:tasks.ns1:task['@done']\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"type\"}]},\"$.ns1:projects.ns1:project.ns1:tasks.ns1:task['@id']\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"integer\"}]},\"$.ns1:projects.ns1:project.ns1:tasks.ns1:task['@name']\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"type\"}]},\"$.ns1:projects.ns1:project['@id']\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"integer\"}]},\"$.ns1:projects.ns1:project['@name']\":{\"combine\":\"AND\",\"matchers\":[{\"match\":\"type\"}]}}},\"metadata\":{\"contentType\":\"application/xml\",\"message-queue-name\":\"message-queue-val\"},\"providerStates\":[{\"name\":\"a functioning FFI interface\"}]}".to_string()));
  let res = pactffi_write_message_pact_file(message_pact_handle.clone(), file_path.as_ptr(), true);
  expect!(res).to(be_eq(0));
}

#[test]
fn pactffi_verifier_cli_args_test() {
    let data = pactffi_verifier_cli_args();
    let c_str: &CStr = unsafe { CStr::from_ptr(data) };
    let str_slice: &str = c_str.to_str().unwrap();

    let options_flags: OptionsFlags = serde_json::from_str(str_slice).unwrap();

    assert!(options_flags.options.len() > 0);
    assert!(options_flags.flags.len() > 0);
}

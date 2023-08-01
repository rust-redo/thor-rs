use regex::Regex;
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};

#[napi(string_enum)]
 pub enum NpmAgent {
  yarn,
  pnpm,
  npm
 }

#[napi(object)]
 pub struct  NpmAgentConfig {
  pub agent: NpmAgent,
  pub version: String
 }

#[napi]
 pub fn get_npm_agent(npm_config_user_agent: String) -> NpmAgentConfig {
  let caps = Regex::new("^(yarn|pnpm|npm)/(\\S+)").unwrap().captures(&npm_config_user_agent).unwrap();
  let agent = &caps[1];

  return NpmAgentConfig {
    agent: match agent {
      "yarn" => NpmAgent::yarn,
      "pnpm" => NpmAgent::pnpm,
      _ => NpmAgent::npm
    },
    version: caps[2].to_string()
  }
 }
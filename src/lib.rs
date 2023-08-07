use napi_derive::napi;

#[napi(object)]
pub struct RscssOptions {
  pub rule: String,
}

#[napi(object)]
pub struct RscssGenerator {
  pub options: RscssOptions,
}

#[napi(object)]
impl RscssGenerator {
  pub fn new(options: RscssOptions) -> RscssGenerator {
    return RscssGenerator { options };
  }

  pub fn generate(&self, input: &str) {
  }

  pub fn parse_token(&self, token: &str) {

  }

  pub fn apply_extractors<'a>(&'a self, input: &'a str) -> Vec<&str> {

  }
}

#[napi]
pub fn create_generator(options: RscssOptions) -> RscssGenerator {
  return RscssGenerator {
    options
  }
}

use std::sync::Arc;

#[derive(Clone, Default)]
pub struct ParseHooks {
  pub import_alias: Option<Arc<dyn Fn(String, String) -> Result<String, String> + Send + Sync>>,
}

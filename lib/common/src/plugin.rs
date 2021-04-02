use crate::application::Builder;
use std::any::Any;

pub trait Plugin: Any + Send + Sync {
  fn build(&self, app: &mut Builder);
  fn name(&self) -> &str {
    std::any::type_name::<Self>()
  }
}

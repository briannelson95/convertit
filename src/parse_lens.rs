use druid::{Lens};
use crate::AppState;

pub struct RoundLens;

impl Lens<AppState, f64> for RoundLens {
    fn with<V, F: FnOnce(&f64) -> V>(&self, data: &AppState, f: F) -> V {
        let value = data.quality.parse::<f64>().unwrap_or(90.0);
        f(&value)
    }

    fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut AppState, f: F) -> V {
        let mut value = data.quality.parse::<f64>().unwrap_or(90.0);
        let result = f(&mut value);
        data.quality = value.round().to_string();
        result
    }
}

use crate::error::AppError;
use crate::plugin::plugin::{EndurancePlugin, FeatureKind};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

pub struct PluginService {
    plugins: Arc<Vec<Arc<EndurancePlugin>>>,
    plugin_cache: Arc<RwLock<HashMap<String, Arc<EndurancePlugin>>>>,
}

impl PluginService {
    pub fn new(plugins: Arc<Vec<Arc<EndurancePlugin>>>) -> Self {
        Self {
            plugins,
            plugin_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn resolve_plugin(&self, rms_account_name: &str) -> Result<Arc<EndurancePlugin>, AppError> {
        tracing::debug!("Resolving plugin: {}", rms_account_name);
        // fast lookup
        {
            let c = self.plugin_cache.read().unwrap();
            let is_empty = c.is_empty();
            if (!is_empty) && let Some(plugin) = c.get(&rms_account_name.to_string()) {
                return Ok(plugin.clone());
            }
        }

        self.plugins
            .iter()
            .find(|p| p.rms_account_name == rms_account_name)
            .inspect(|plugin| {
                self.plugin_cache
                    .write()
                    .unwrap()
                    .insert(rms_account_name.to_string(), plugin.deref().clone());
            })
            .cloned()
            .ok_or_else(|| AppError::PluginNotFound(rms_account_name.to_string()))
    }

    pub fn get_booking_with_feature(
        &self,
        rms_account_name: &str,
        feature_kind: &FeatureKind,
    ) -> Result<Arc<EndurancePlugin>, AppError> {
        self.plugins
            .iter()
            .find(|p| p.rms_account_name == rms_account_name && p.has_feature(feature_kind))
            .cloned()
            .ok_or_else(|| AppError::PluginNotFound(rms_account_name.to_string()))
    }
}

use std::sync::Arc;
use crate::error::AppError;
use crate::plugin::plugin::EndurancePlugin;

pub struct PluginService {
    plugins: Arc<Vec<Arc<EndurancePlugin>>>,
}

impl PluginService {
    
    pub fn new(plugins: Arc<Vec<Arc<EndurancePlugin>>>) -> Self {
        Self {
            plugins
        }
    }
    pub fn resolve_plugin(&self, rms: &str) -> Result<Arc<EndurancePlugin>, AppError> {
        self.plugins
            .iter()
            .find(|p| p.rms == rms)
            .cloned()
            .ok_or_else(|| AppError::PluginNotFound(rms.to_string()))
    }
}
//! Providers Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Provider {
    pub(crate) id: i16,
    pub(crate) name: String,
    pub(crate) display_name: String,
    pub(crate) is_oauth: bool,
    pub(crate) is_active: bool,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderType {
    Local,
    Google,
    GitHub,
    Microsoft,
    Apple,
    Facebook,
    Twitter,
    Discord,
    Custom(String),
}

impl ProviderType {
    pub fn as_str(&self) -> &str {
        match self {
            ProviderType::Local => "local",
            ProviderType::Google => "google",
            ProviderType::GitHub => "github",
            ProviderType::Microsoft => "microsoft",
            ProviderType::Apple => "apple",
            ProviderType::Facebook => "facebook",
            ProviderType::Twitter => "twitter",
            ProviderType::Discord => "discord",
            ProviderType::Custom(name) => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ProviderType::Local => "Local Account",
            ProviderType::Google => "Google",
            ProviderType::GitHub => "GitHub",
            ProviderType::Microsoft => "Microsoft",
            ProviderType::Apple => "Apple",
            ProviderType::Facebook => "Facebook",
            ProviderType::Twitter => "Twitter",
            ProviderType::Discord => "Discord",
            ProviderType::Custom(_) => "Custom Provider",
        }
    }

    pub fn is_oauth(&self) -> bool {
        !matches!(self, ProviderType::Local)
    }
}

impl Provider {
    /// Create new provider
    pub(crate) fn new(
        id: i16,
        provider_type: ProviderType,
        custom_display_name: Option<String>,
    ) -> Self {
        let name = provider_type.as_str().to_string();
        let display_name = custom_display_name
            .unwrap_or_else(|| provider_type.display_name().to_string());
        let is_oauth = provider_type.is_oauth();
        
        info!("Creating new provider: {} ({})", name, display_name);
        
        Self {
            id,
            name,
            display_name,
            is_oauth,
            is_active: true,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create local provider (password-based authentication)
    pub(crate) fn local(id: i16) -> Self {
        Self::new(id, ProviderType::Local, None)
    }

    /// Create OAuth provider
    pub(crate) fn oauth(id: i16, provider_type: ProviderType) -> Self {
        Self::new(id, provider_type, None)
    }

    /// Activate provider
    pub(crate) fn activate(&mut self) {
        if !self.is_active {
            self.is_active = true;
            info!("Activated provider: {}", self.name);
        }
    }

    /// Deactivate provider
    pub(crate) fn deactivate(&mut self) {
        if self.is_active {
            self.is_active = false;
            warn!("Deactivated provider: {}", self.name);
        }
    }

    /// Update display name
    pub(crate) fn update_display_name(&mut self, new_display_name: String) {
        self.display_name = new_display_name.clone();
        info!("Updated display name for provider {}: {}", self.name, new_display_name);
    }

    /// Check if provider is available for new registrations
    pub(crate) fn is_available(&self) -> bool {
        self.is_active
    }

    /// Check if provider supports OAuth
    pub(crate) fn supports_oauth(&self) -> bool {
        self.is_oauth
    }

    /// Check if provider is local (password-based)
    pub(crate) fn is_local(&self) -> bool {
        !self.is_oauth && self.name == "local"
    }

    /// Get provider type from name
    pub(crate) fn get_provider_type(&self) -> ProviderType {
        match self.name.as_str() {
            "local" => ProviderType::Local,
            "google" => ProviderType::Google,
            "github" => ProviderType::GitHub,
            "microsoft" => ProviderType::Microsoft,
            "apple" => ProviderType::Apple,
            "facebook" => ProviderType::Facebook,
            "twitter" => ProviderType::Twitter,
            "discord" => ProviderType::Discord,
            _ => ProviderType::Custom(self.name.clone()),
        }
    }

    /// Check if provider requires additional configuration
    pub(crate) fn requires_config(&self) -> bool {
        self.is_oauth // OAuth providers typically need client_id, client_secret, etc.
    }

    /// Get public provider data (safe for API responses)
    pub(crate) fn to_public(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            is_oauth: self.is_oauth,
            is_active: self.is_active,
            created_at: self.created_at,
        }
    }

    // Public accessors
    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn is_oauth(&self) -> bool {
        self.is_oauth
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

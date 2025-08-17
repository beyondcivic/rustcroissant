//! Version information module for rustcroissant

/// Struct containing application version information
pub struct Version {
    /// Name of the application
    pub app_name: &'static str,

    /// Service version
    pub version: &'static str,

    /// Git hash of the commit the service is built from
    pub git_hash: &'static str,

    /// Build time in RFC3339 format
    pub build_time: &'static str,
}

// Default values - will be overridden during build
pub static APP_NAME: &str = "rustcroissant";
pub static VERSION: &str = "dev";
pub static GIT_HASH: &str = "dev";
pub static BUILD_TIME: &str = "now";

/// Get the current version information
pub fn get_version() -> Version {
    Version {
        app_name: APP_NAME,
        version: VERSION,
        git_hash: GIT_HASH,
        build_time: BUILD_TIME,
    }
}

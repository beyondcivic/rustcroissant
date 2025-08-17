//! Version information module for rustcroissant

// Include generated version info from build.rs
include!(concat!(env!("OUT_DIR"), "/version_info.rs"));

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

/// Get the current version information
pub fn get_version() -> Version {
    Version {
        app_name: APP_NAME,
        version: VERSION,
        git_hash: GIT_HASH,
        build_time: BUILD_TIME,
    }
}

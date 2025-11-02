//! Runtime support for checking versions and feature availability.

use std::sync::LazyLock;

// Query for curl version info just once since it is immutable.
static CURL_VERSION: LazyLock<curl::Version> = LazyLock::new(curl::Version::get);

/// Check if runtime support is available for the given HTTP version.
///
/// This only indicates whether support for communicating with this HTTP version
/// is available, which is usually determined by which features were enabled
/// during compilation, but can also be affected by what is available in system
/// libraries when using dynamic linking.
///
/// This does not indicate which versions Isahc will attempt to use by default.
/// To customize which versions to use within a particular client or request
/// instance, see [`VersionNegotiation`][crate::config::VersionNegotiation].
pub fn is_http_version_supported(version: http::Version) -> bool {
    match version {
        // HTTP/0.9 was disabled by default as of 7.66.0. See also
        // https://github.com/sagebind/isahc/issues/310 if we ever decide to
        // allow enabling it again.
        http::Version::HTTP_09 => match curl_version() {
            (7, minor, _) if minor < 66 => true,
            (major, _, _) if major < 7 => true,
            _ => false,
        },
        http::Version::HTTP_10 => true,
        http::Version::HTTP_11 => true,
        http::Version::HTTP_2 => CURL_VERSION.feature_http2(),
        http::Version::HTTP_3 => CURL_VERSION.feature_http3(),
        _ => false,
    }
}

fn curl_version() -> (u8, u8, u8) {
    let bits = CURL_VERSION.version_num();

    ((bits >> 16) as u8, (bits >> 8) as u8, bits as u8)
}

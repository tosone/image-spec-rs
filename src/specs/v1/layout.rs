/// IMAGE_LAYOUT_FILE is the file name of oci image layout file
pub const IMAGE_LAYOUT_FILE: &str = "oci-layout";

// IMAGE_LAYOUT_VERSION is the version of ImageLayout
pub const IMAGE_LAYOUT_VERSION: &str = "1.0.0";

/// ImageLayout is the structure in the "oci-layout" file, found in the root
/// of an OCI Image-layout directory.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ImageLayout {
    #[serde(rename = "imageLayoutVersion")]
    pub version: String,
}

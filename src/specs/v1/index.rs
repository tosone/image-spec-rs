/// Index references manifests for various platforms.
/// This structure provides `application/vnd.oci.image.index.v1+json` mediatype when marshalled to JSON.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Index {
    // SchemaVersion is the image manifest schema that this image follows
    #[serde(rename = "SchemaVersion")]
    pub schema_version: isize,

    // MediaType specificies the type of this document data structure e.g. `application/vnd.oci.image.index.v1+json`
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    // Manifests references platform specific manifests.
    #[serde(rename = "manifests")]
    pub manifests: Vec<super::descriptor::Descriptor>,

    // Annotations contains arbitrary metadata for the image index.
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
}

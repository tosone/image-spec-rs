/// Manifest provides `application/vnd.oci.image.manifest.v1+json` mediatype structure when marshalled to JSON.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Manifest {
    /// schema_version is the image manifest schema that this image follows
    #[serde(rename = "SchemaVersion")]
    pub schema_version: isize,

    /// MediaType specificies the type of this document data structure e.g. `application/vnd.oci.image.manifest.v1+json`
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    /// Config references a configuration object for a container, by digest.
    /// The referenced configuration object is a JSON blob that the runtime uses to set up the container.
    #[serde(rename = "config")]
    pub config: super::descriptor::Descriptor,

    /// Layers is an indexed list of layers referenced by the manifest.
    #[serde(rename = "layers")]
    pub layers: Vec<super::descriptor::Descriptor>,

    /// Annotations contains arbitrary metadata for the image manifest.
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
}

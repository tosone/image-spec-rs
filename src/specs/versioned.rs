/// Versioned provides a struct with the manifest schemaVersion and mediaType.
/// Incoming content with unknown schema version can be decoded against this
/// struct to check the version.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Versioned {
    // SchemaVersion is the image manifest schema that this image follows
    #[serde(rename = "SchemaVersion")]
    pub schema_version: isize,
}

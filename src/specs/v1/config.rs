#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Nothing;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ImageConfig {
    /// User defines the username or UID which the process in the container should run as.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// ExposedPorts a set of ports to expose from a container running this image.
    #[serde(rename = "ExposedPorts", skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<std::collections::HashMap<String, Nothing>>,

    /// Env is a list of environment variables to be used in a container.
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    /// Entrypoint defines a list of arguments to use as the command to execute when the container starts.
    #[serde(rename = "Entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,

    /// Cmd defines the default arguments to the entrypoint of the container.
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,

    /// Volumes is a set of directories describing where the process is likely write data specific to a container instance.
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<std::collections::HashMap<String, Nothing>>,

    /// WorkingDir sets the current working directory of the entrypoint process in the container.
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    /// Labels contains arbitrary metadata for the container.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    /// StopSignal contains the system call signal that will be sent to the container to exit.
    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
}

/// RootFS describes a layer content addresses
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct RootFS {
    // Type is the type of the rootfs.
    #[serde(rename = "type")]
    pub type_: String,

    // DiffIDs is an array of layer content hashes (DiffIDs), in order from bottom-most to top-most.
    #[serde(rename = "diff_ids")]
    pub diff_ids: Vec<String>,
}

// History describes the history of a layer.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct History {
    // Created is the combined date and time at which the layer was created, formatted as defined by RFC 3339, section 5.6.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,

    // CreatedBy is the command which created the layer.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    // Author is the author of the build point.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    // Comment is a custom message set when creating the layer.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    // EmptyLayer is used to mark if the history item created a filesystem diff.
    #[serde(rename = "empty_layer", skip_serializing_if = "Option::is_none")]
    pub empty_layer: Option<bool>,
}

/// Image is the JSON structure which describes some basic information about the image.
/// This provides the `application/vnd.oci.image.config.v1+json` mediatype when marshalled to JSON.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Image {
    // Created is the combined date and time at which the image was created, formatted as defined by RFC 3339, section 5.6.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,

    // Author defines the name and/or email address of the person or entity which created and is responsible for maintaining the image.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    // Architecture is the CPU architecture which the binaries in this image are built to run on.
    #[serde(rename = "architecture")]
    pub architecture: String,

    // Variant is the variant of the specified CPU architecture which image binaries are intended to run on.
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,

    // OS is the name of the operating system which the image is built to run on.
    #[serde(rename = "os")]
    pub os: String,

    // OSVersion is an optional field specifying the operating system
    // version, for example on Windows `10.0.14393.1066`.
    #[serde(rename = "os.version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,

    // OSFeatures is an optional field specifying an array of strings,
    // each listing a required OS feature (for example on Windows `win32k`).
    #[serde(rename = "os.features", skip_serializing_if = "Option::is_none")]
    pub os_features: Option<Vec<String>>,

    // Config defines the execution parameters which should be used as a base when running a container using the image.
    // #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    // pub config: Option<ImageConfig>,
    /// RootFS references the layer content addresses used by the image.
    #[serde(rename = "rootfs")]
    pub rootfs: RootFS,

    /// History describes the history of each layer.
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<History>>,
}

/// MEDIA_TYPE_DESCRIPTOR specifies the media type for a content descriptor.
pub const MEDIA_TYPE_DESCRIPTOR: &str = "application/vnd.oci.descriptor.v1+json";

/// MEDIA_TYPE_LAYOUT_HEADER specifies the media type for the oci-layout.
pub const MEDIA_TYPE_LAYOUT_HEADER: &str = "application/vnd.oci.layout.header.v1+json";

/// MEDIA_TYPE_IMAGE_MANIFEST specifies the media type for an image manifest.
pub const MEDIA_TYPE_IMAGE_MANIFEST: &str = "application/vnd.oci.image.manifest.v1+json";

/// MEDIA_TYPE_IMAGE_INDEX specifies the media type for an image index.
pub const MEDIA_TYPE_IMAGE_INDEX: &str = "application/vnd.oci.image.index.v1+json";

/// MEDIA_TYPE_IMAGE_LAYER is the media type used for layers referenced by the manifest.
pub const MEDIA_TYPE_IMAGE_LAYER: &str = "application/vnd.oci.image.layer.v1.tar";

/// MEDIA_TYPE_IMAGE_LAYER_GZIP is the media type used for gzipped layers
/// referenced by the manifest.
pub const MEDIA_TYPE_IMAGE_LAYER_GZIP: &str = "application/vnd.oci.image.layer.v1.tar+gzip";

/// MEDIA_TYPE_IMAGE_LAYER_ZSTD is the media type used for zstd compressed
/// layers referenced by the manifest.
pub const MEDIA_TYPE_IMAGE_LAYER_ZSTD: &str = "application/vnd.oci.image.layer.v1.tar+zstd";

/// MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE is the media type for layers referenced by
/// the manifest but with distribution restrictions.
pub const MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar";

/// MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE_GZIP is the media type for
/// gzipped layers referenced by the manifest but with distribution
/// restrictions.
pub const MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE_GZIP: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip";

/// MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE_ZSTD is the media type for zstd
/// compressed layers referenced by the manifest but with distribution
/// restrictions.
pub const MEDIA_TYPE_IMAGE_LAYER_NON_DISTRIBUTABLE_ZSTD: &str =
    "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd";

/// MEDIA_TYPE_IMAGE_CONFIG specifies the media type for the image configuration.
pub const MEDIA_TYPE_IMAGE_CONFIG: &str = "application/vnd.oci.image.config.v1+json";

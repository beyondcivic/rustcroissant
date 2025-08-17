use chrono::{DateTime, FixedOffset, NaiveDate};
use serde;
use serde::{Deserialize, Serialize};
// ============================================================================
// Core Croissant Structures
// ============================================================================

/// Field represents a field in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    pub source: FieldSource,
}

/// FieldSource represents the source information for a field
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldSource {
    pub extract: Extract,
    #[serde(rename = "fileObject")]
    pub file_object: FileObject,
}

/// Extract represents the extraction information for a field source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Extract {
    pub column: String,
}

/// FileObject represents a file object reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileObject {
    #[serde(rename = "@id")]
    pub id: String,
}

/// Distribution represents a file in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Distribution {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
    #[serde(rename = "contentSize")]
    pub content_size: String,
    #[serde(rename = "contentUrl")]
    pub content_url: String,
    #[serde(rename = "encodingFormat")]
    pub encoding_format: String,
    pub sha256: String,
}

/// RecordSet represents a record set in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordSet {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
    pub description: String,
    pub field: Vec<Field>,
}

/// Context represents the JSON-LD context in the Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Context {
    #[serde(rename = "@language")]
    pub language: String,
    #[serde(rename = "@vocab")]
    pub vocab: String,
    #[serde(rename = "citeAs")]
    pub cite_as: String,
    pub column: String,
    #[serde(rename = "conformsTo")]
    pub conforms_to: String,
    pub cr: String,
    pub dct: String,
    pub data: DataContext,
    #[serde(rename = "dataType")]
    pub data_type: DataTypeContext,
    pub extract: String,
    pub field: String,
    #[serde(rename = "fileObject")]
    pub file_object: String,
    #[serde(rename = "fileProperty")]
    pub file_property: String,
    pub sc: String,
    pub source: String,
}

/// DataContext represents the data field in the context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataContext {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
}

/// DataTypeContext represents the dataType field in the context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataTypeContext {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Metadata represents the complete Croissant metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    #[serde(rename = "@context")]
    pub context: Context,
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "conformsTo")]
    pub conforms_to: String,
    #[serde(rename = "datePublished")]
    pub date_published: String,
    pub version: String,
    pub distribution: Vec<Distribution>,
    #[serde(rename = "recordSet")]
    pub record_set: Vec<RecordSet>,
}

// ============================================================================
// Data Type Inference
// ============================================================================

/// Supported data types for Croissant fields
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Text,
    Date,
    Boolean,
}

impl DataType {
    /// Convert to schema.org data type string
    pub fn to_schema_org(&self) -> &'static str {
        match self {
            DataType::Integer => "sc:Integer",
            DataType::Float => "sc:Float",
            DataType::Text => "sc:Text",
            DataType::Date => "sc:Date",
            DataType::Boolean => "sc:Boolean",
        }
    }
}

/// Infer the data type from a value string
pub fn infer_data_type(value: &str) -> DataType {
    let trimmed = value.trim();

    // Try to parse as integer
    if trimmed.parse::<i64>().is_ok() {
        return DataType::Integer;
    }

    // Try to parse as float
    if trimmed.parse::<f64>().is_ok() {
        return DataType::Float;
    }

    // Try to parse as boolean
    if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("false") {
        return DataType::Boolean;
    }

    // Try to parse as date (YYYY-MM-DD)
    if chrono::NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").is_ok() {
        return DataType::Date;
    }

    // Try to parse as ISO 8601 datetime
    if DateTime::parse_from_rfc3339(trimmed).is_ok() {
        return DataType::Date;
    }

    // Default to Text
    DataType::Text
}

// ============================================================================
// Context Creation
// ============================================================================

/// Create the default context for Croissant metadata
pub fn create_default_context() -> Context {
    Context {
        language: "en".to_string(),
        vocab: "https://schema.org/".to_string(),
        cite_as: "cr:citeAs".to_string(),
        column: "cr:column".to_string(),
        conforms_to: "dct:conformsTo".to_string(),
        cr: "http://mlcommons.org/croissant/".to_string(),
        dct: "http://purl.org/dc/terms/".to_string(),
        data: DataContext {
            id: "cr:data".to_string(),
            type_: "@json".to_string(),
        },
        data_type: DataTypeContext {
            id: "cr:dataType".to_string(),
            type_: "@vocab".to_string(),
        },
        extract: "cr:extract".to_string(),
        field: "cr:field".to_string(),
        file_object: "cr:fileObject".to_string(),
        file_property: "cr:fileProperty".to_string(),
        sc: "https://schema.org/".to_string(),
        source: "cr:source".to_string(),
    }
}

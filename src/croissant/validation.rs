//! Validation logic for Croissant metadata
use crate::croissant::core::Metadata;
use crate::croissant::core::RecordSet;
use crate::croissant::errors::{Error, Result};
use std::collections::HashSet;
use std::path::Path;

/// Issue severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum IssueSeverity {
    Error,
    Warning,
}

/// A single validation issue
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub message: String,
    pub context: Option<String>,
}

impl ValidationIssue {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Error,
            message: message.into(),
            context: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Warning,
            message: message.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Collection of validation issues
#[derive(Debug, Clone)]
pub struct ValidationIssues {
    issues: Vec<ValidationIssue>,
}

impl ValidationIssues {
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    pub fn add_error(&mut self, message: impl Into<String>) {
        self.issues.push(ValidationIssue::error(message));
    }

    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.issues.push(ValidationIssue::warning(message));
    }

    pub fn add_error_with_context(
        &mut self,
        message: impl Into<String>,
        context: impl Into<String>,
    ) {
        self.issues
            .push(ValidationIssue::error(message).with_context(context));
    }

    pub fn add_warning_with_context(
        &mut self,
        message: impl Into<String>,
        context: impl Into<String>,
    ) {
        self.issues
            .push(ValidationIssue::warning(message).with_context(context));
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == IssueSeverity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == IssueSeverity::Warning)
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Warning)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    /// Generate a human-readable report of all issues
    pub fn report(&self) -> String {
        if self.issues.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let errors: Vec<_> = self
            .issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Error)
            .collect();
        let warnings: Vec<_> = self
            .issues
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Warning)
            .collect();

        if !errors.is_empty() {
            result.push_str(&format!(
                "Found the following {} error(s) during the validation:\n",
                errors.len()
            ));
            for issue in errors {
                if let Some(ref context) = issue.context {
                    result.push_str(&format!("  -  [{}] {}\n", context, issue.message));
                } else {
                    result.push_str(&format!("  -  {}\n", issue.message));
                }
            }
        }

        if !warnings.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&format!(
                "Found the following {} warning(s) during the validation:\n",
                warnings.len()
            ));
            for issue in warnings {
                if let Some(ref context) = issue.context {
                    result.push_str(&format!("  -  [{}] {}\n", context, issue.message));
                } else {
                    result.push_str(&format!("  -  {}\n", issue.message));
                }
            }
        }

        result.trim_end().to_string()
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }
}

impl Default for ValidationIssues {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate a Croissant metadata file
pub fn validate_file(file_path: &Path) -> Result<ValidationIssues> {
    let content =
        std::fs::read_to_string(file_path).map_err(|_| Error::file_not_found(file_path))?;

    let metadata: Metadata = serde_json::from_str(&content)?;
    Ok(validate_metadata(&metadata))
}

/// Validate Croissant metadata structure
pub fn validate_metadata(metadata: &Metadata) -> ValidationIssues {
    let mut issues = ValidationIssues::new();

    validate_metadata_basic(&mut issues, metadata);
    validate_distributions(&mut issues, metadata);
    validate_record_sets(&mut issues, metadata);
    validate_references(&mut issues, metadata);

    issues
}

fn validate_metadata_basic(issues: &mut ValidationIssues, metadata: &Metadata) {
    let context = format!("Metadata({})", metadata.name);

    // Validate required fields
    if metadata.name.is_empty() {
        issues.add_error_with_context(
            "Property \"https://schema.org/name\" is mandatory, but does not exist.",
            &context,
        );
    }

    // Validate type
    if metadata.type_ != "sc:Dataset" {
        issues.add_error_with_context(
            "The current JSON-LD doesn't extend https://schema.org/Dataset.",
            &context,
        );
    }

    // Validate conformsTo is set
    if metadata.conforms_to.is_empty() {
        issues.add_warning_with_context(
            "Property \"http://purl.org/dc/terms/conformsTo\" is recommended, but does not exist.",
            &context,
        );
    }

    // Validate description
    if metadata.description.is_empty() {
        issues.add_warning_with_context(
            "Property \"https://schema.org/description\" is recommended, but does not exist.",
            &context,
        );
    }
}

fn validate_distributions(issues: &mut ValidationIssues, metadata: &Metadata) {
    for distribution in &metadata.distribution {
        let context = format!(
            "Metadata({}) > FileObject({})",
            metadata.name, distribution.name
        );

        // Validate required fields
        if distribution.name.is_empty() {
            issues.add_error_with_context(
                "Property \"https://schema.org/name\" is mandatory, but does not exist.",
                &context,
            );
        }

        // Validate type
        if distribution.type_ != "cr:FileObject" && distribution.type_ != "cr:FileSet" {
            issues.add_error_with_context(
                format!(
                    "\"{}\" should have an attribute \"@type\": \"http://mlcommons.org/croissant/FileObject\" or \"@type\": \"http://mlcommons.org/croissant/FileSet\". Got {} instead.",
                    distribution.name,
                    distribution.type_
                ),
                &context
            );
        }

        // Validate content URL
        if distribution.content_url.is_empty() {
            issues.add_error_with_context(
                "Property \"https://schema.org/contentUrl\" is mandatory, but does not exist.",
                &context,
            );
        }

        // Validate encoding format
        if distribution.encoding_format.is_empty() {
            issues.add_error_with_context(
                "Property \"https://schema.org/encodingFormat\" is mandatory, but does not exist.",
                &context,
            );
        }

        // Validate SHA256
        if distribution.sha256.is_empty() {
            issues.add_warning_with_context(
                "Property \"https://schema.org/sha256\" is recommended for file integrity verification.",
                &context
            );
        } else if distribution.sha256.len() != 64
            || !distribution.sha256.chars().all(|c| c.is_ascii_hexdigit())
        {
            issues.add_error_with_context(
                "Invalid SHA256 hash format. Expected 64 hexadecimal characters.",
                &context,
            );
        }
    }
}

fn validate_record_sets(issues: &mut ValidationIssues, metadata: &Metadata) {
    for record_set in &metadata.record_set {
        let context = format!(
            "Metadata({}) > RecordSet({})",
            metadata.name, record_set.name
        );

        // Validate required fields
        if record_set.name.is_empty() {
            issues.add_error_with_context(
                "Property \"https://schema.org/name\" is mandatory, but does not exist.",
                &context,
            );
        }

        // Validate type
        if record_set.type_ != "cr:RecordSet" {
            issues.add_error_with_context(
                format!(
                    "\"{}\" should have an attribute \"@type\": \"http://mlcommons.org/croissant/RecordSet\". Got {} instead.",
                    record_set.name,
                    record_set.type_
                ),
                &context
            );
        }

        // Validate fields
        validate_fields(issues, metadata, record_set);
    }
}

fn validate_fields(issues: &mut ValidationIssues, metadata: &Metadata, record_set: &RecordSet) {
    for field in &record_set.field {
        let context = format!(
            "Metadata({}) > RecordSet({}) > Field({})",
            metadata.name, record_set.name, field.name
        );

        // Validate required fields
        if field.name.is_empty() {
            issues.add_error_with_context(
                "Property \"https://schema.org/name\" is mandatory, but does not exist.",
                &context,
            );
        }

        // Validate type
        if field.type_ != "cr:Field" {
            issues.add_error_with_context(
                format!(
                    "\"{}\" should have an attribute \"@type\": \"http://mlcommons.org/croissant/Field\". Got {} instead.",
                    field.name,
                    field.type_
                ),
                &context
            );
        }

        // Validate data type
        if field.data_type.is_empty() {
            issues.add_error_with_context(
                format!(
                    "The field does not specify a valid http://mlcommons.org/croissant/dataType, neither does any of its predecessor. Got: {}",
                    field.data_type
                ),
                &context
            );
        } else {
            validate_data_type(&field.data_type, issues, &context);
        }

        // Validate source
        if field.source.extract.column.is_empty() || field.source.file_object.id.is_empty() {
            issues.add_error_with_context(
                format!(
                    "Node \"{}\" is a field and has no source. Please, use http://mlcommons.org/croissant/source to specify the source.",
                    field.id
                ),
                &context
            );
        }
    }
}

fn validate_data_type(data_type: &str, issues: &mut ValidationIssues, context: &str) {
    let valid_types = [
        "sc:Text",
        "sc:Integer",
        "sc:Float",
        "sc:Boolean",
        "sc:Date",
        "sc:DateTime",
        "sc:Time",
        "sc:URL",
        "sc:Number",
    ];

    if !valid_types.contains(&data_type) {
        issues.add_warning_with_context(
            format!(
                "Unknown data type: {}. Consider using a standard schema.org type.",
                data_type
            ),
            context,
        );
    }
}

fn validate_references(issues: &mut ValidationIssues, metadata: &Metadata) {
    // Collect all distribution IDs
    let distribution_ids: HashSet<_> = metadata
        .distribution
        .iter()
        .map(|dist| dist.id.as_str())
        .collect();

    // Validate field references to file objects
    for record_set in &metadata.record_set {
        for field in &record_set.field {
            let file_object_id = &field.source.file_object.id;
            if !file_object_id.is_empty() && !distribution_ids.contains(file_object_id.as_str()) {
                let context = format!(
                    "Metadata({}) > RecordSet({}) > Field({})",
                    metadata.name, record_set.name, field.name
                );
                issues.add_error_with_context(
                    format!(
                        "Field references non-existent file object: {}",
                        file_object_id
                    ),
                    &context,
                );
            }
        }
    }
}

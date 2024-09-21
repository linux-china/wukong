use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct JBangCatalog {
    pub catalogs: Option<HashMap<String, CatalogRef>>,
    pub aliases: Option<HashMap<String, Alias>>,
    pub templates: Option<HashMap<String, Template>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Alias {
    #[serde(rename = "script-ref")]
    pub script_ref: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CatalogRef {
    #[serde(rename = "catalog-ref")]
    pub catalog_ref: String,
    pub description: Option<String>,
    #[serde(rename = "import")]
    pub import_items: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Template {
    pub file_refs: HashMap<String, String>,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, TemplateProperty>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateProperty {
    pub description: String,
    #[serde(rename = "default")]
    pub default_value: Option<String>,
}

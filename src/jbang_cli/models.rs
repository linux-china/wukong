use std::path::Path;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::jbang_cli::jbang_home;

/// JBang catalog file. Only the fields wukong uses are modeled explicitly; everything else
/// (e.g. `base-ref`) is kept in `extra` so a read-modify-write round trip never drops data.
/// `IndexMap` keeps the key order of the original file to avoid noisy diffs.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JBangCatalog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<IndexMap<String, CatalogRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<IndexMap<String, Alias>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<IndexMap<String, Template>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl JBangCatalog {
    pub fn add_alias(&mut self, name: &str, alias: Alias) {
        self.aliases.get_or_insert_with(IndexMap::new).insert(name.to_string(), alias);
    }

    pub fn remove_alias(&mut self, name: &str) {
        if let Some(aliases) = &mut self.aliases {
            aliases.shift_remove(name);
        }
    }

    pub fn add_catalog(&mut self, name: &str, catalog: CatalogRef) {
        self.catalogs.get_or_insert_with(IndexMap::new).insert(name.to_string(), catalog);
    }

    pub fn remove_catalog(&mut self, name: &str) {
        if let Some(catalogs) = &mut self.catalogs {
            catalogs.shift_remove(name);
        }
    }

    pub fn add_template(&mut self, name: &str, template: Template) {
        self.templates.get_or_insert_with(IndexMap::new).insert(name.to_string(), template);
    }

    pub fn remove_template(&mut self, name: &str) {
        if let Some(templates) = &mut self.templates {
            templates.shift_remove(name);
        }
    }

    pub fn write<P: AsRef<Path>>(&self, catalog_file: P) {
        serde_json::to_writer_pretty(std::fs::File::create(catalog_file).unwrap(), self).unwrap();
    }

    pub fn write_default(&self) {
        self.write(jbang_home().join("jbang-catalog.json"));
    }
}

/// Alias entry; unmodeled fields such as `arguments`, `java-options`, `dependencies`
/// are preserved in `extra`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Alias {
    #[serde(rename = "script-ref")]
    pub script_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CatalogRef {
    #[serde(rename = "catalog-ref")]
    pub catalog_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "import")]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub import_items: bool,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "file-refs")]
    pub file_refs: IndexMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, TemplateProperty>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::jbang_cli::jbang_home;
    use super::*;

    #[test]
    fn test_read_default_jbang_catalog() {
        let jbang_catalog_json = jbang_home().join("jbang-catalog.json");
        let catalog: JBangCatalog = serde_json::from_reader(File::open(jbang_catalog_json).unwrap()).unwrap();
        println!("{:?}", catalog);
    }

    #[test]
    fn test_round_trip_preserves_unknown_fields_and_order() {
        let json = r#"{
  "base-ref": "https://example.com",
  "catalogs": {},
  "aliases": {
    "zeta": {
      "script-ref": "z.java",
      "arguments": ["a", "b"],
      "java-options": ["-Xmx1g"],
      "dependencies": ["g:a:1"]
    },
    "alpha": {
      "script-ref": "a.java",
      "description": "Alpha"
    }
  }
}"#;
        let mut catalog: JBangCatalog = serde_json::from_str(json).unwrap();
        catalog.add_alias("mid", Alias { script_ref: "m.java".to_string(), ..Default::default() });
        catalog.remove_alias("mid");
        let output = serde_json::to_string_pretty(&catalog).unwrap();
        let expected: Value = serde_json::from_str(json).unwrap();
        let actual: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(expected, actual);
        assert!(output.find("zeta").unwrap() < output.find("alpha").unwrap());
    }
}

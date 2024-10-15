use std::collections::HashMap;
use std::path::{Path};
use serde::{Deserialize, Serialize};
use crate::jbang_cli::jbang_home;

#[derive(Debug, Serialize, Deserialize)]
pub struct JBangCatalog {
    pub catalogs: Option<HashMap<String, CatalogRef>>,
    pub aliases: Option<HashMap<String, Alias>>,
    pub templates: Option<HashMap<String, Template>>,
}

impl JBangCatalog {
    pub fn add_alias(&mut self, name: &str, alias: Alias) {
        if let Some(aliases) = &mut self.aliases {
            aliases.insert(name.to_string(), alias);
        } else {
            let mut aliases = HashMap::new();
            aliases.insert(name.to_string(), alias);
            self.aliases = Some(aliases);
        }
    }

    pub fn remove_alias(&mut self, name: &str) {
        if let Some(aliases) = &mut self.aliases {
            aliases.remove(name);
        }
    }

    pub fn add_catalog(&mut self, name: &str, catalog: CatalogRef) {
        if let Some(catalogs) = &mut self.catalogs {
            catalogs.insert(name.to_string(), catalog);
        } else {
            let mut catalogs = HashMap::new();
            catalogs.insert(name.to_string(), catalog);
            self.catalogs = Some(catalogs);
        }
    }

    pub fn remove_catalog(&mut self, name: &str) {
        if let Some(catalogs) = &mut self.catalogs {
            catalogs.remove(name);
        }
    }

    pub fn add_template(&mut self, name: &str, template: Template) {
        if let Some(templates) = &mut self.templates {
            templates.insert(name.to_string(), template);
        } else {
            let mut templates = HashMap::new();
            templates.insert(name.to_string(), template);
            self.templates = Some(templates);
        }
    }

    pub fn remove_template(&mut self, name: &str) {
        if let Some(templates) = &mut self.templates {
            templates.remove(name);
        }
    }

    pub fn write<P: AsRef<Path>>(&self, catalog_file: P) {
        serde_json::to_writer_pretty(std::fs::File::create(catalog_file).unwrap(), self).unwrap();
    }

    pub fn write_default(&self) {
        self.write(jbang_home().join("jbang-catalog.json"));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    #[serde(rename = "script-ref")]
    pub script_ref: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogRef {
    #[serde(rename = "catalog-ref")]
    pub catalog_ref: String,
    pub description: Option<String>,
    #[serde(rename = "import")]
    #[serde(default = "bool::default")]
    pub import_items: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "file-refs")]
    pub file_refs: HashMap<String, String>,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, TemplateProperty>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateProperty {
    pub description: String,
    #[serde(rename = "default")]
    pub default_value: Option<String>,
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
}

use std::collections::HashMap;
use std::fs;
use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};
use crate::mt_cli::m2_dir;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "toolchains")]
pub struct Toolchains {
    pub toolchain: Vec<Toolchain>,
}

impl Toolchains {
    pub fn load() -> Self {
        let file = m2_dir().join("toolchains.xml");
        let reader = std::fs::File::open(file).unwrap();
        quick_xml::de::from_reader(std::io::BufReader::new(reader)).unwrap()
    }

    pub fn add_jdk(&mut self, version: &str, vendor: Option<String>, jdk_home: String) {
        let toolchain = Toolchain {
            type_: "jdk".to_string(),
            provides: Provides {
                version: version.to_string(),
                vendor,
            },
            configuration: [("jdkHome".to_string(), jdk_home)].iter().cloned().collect(),
        };
        self.toolchain.push(toolchain);
    }

    pub fn remove_jdk(&mut self, version: &str, vendor: Option<String>) {
        let index = self.toolchain.iter().position(|t| {
            t.type_ == "jdk" && t.provides.version == version && t.provides.vendor == vendor
        });
        if let Some(index) = index {
            self.toolchain.remove(index);
        }
    }

    pub fn write(&self) {
        let file = m2_dir().join("toolchains.xml");
        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 2);
        self.serialize(ser).unwrap();
        fs::write(file, buffer).unwrap();
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Toolchain {
    #[serde(rename = "type")]
    pub type_: String,
    pub provides: Provides,
    pub configuration: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Provides {
    pub version: String,
    pub vendor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toolchains() {
        let toolchains = Toolchains::load();
        println!("{:?}", toolchains);
        toolchains.write();
    }
}

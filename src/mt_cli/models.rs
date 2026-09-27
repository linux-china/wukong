use crate::mt_cli::m2_dir;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;

/// Maven `~/.m2/toolchains.xml`.
///
/// The file is kept as raw text and only the affected `<toolchain>` element (or `<jdkHome>` text)
/// is changed on add/remove, so comments, namespaces, `<provides><id>` and any other fields
/// maintained by hand are preserved.
#[derive(Debug, PartialEq)]
pub struct Toolchains {
    content: String,
    pub toolchain: Vec<Toolchain>,
    /// insert position for new toolchains: start of `</toolchains>`, or span of `<toolchains/>`
    root_end: Option<RootEnd>,
}

#[derive(Debug, PartialEq)]
enum RootEnd {
    Close(usize),
    Empty(Range<usize>),
}

#[derive(Debug, PartialEq)]
pub struct Toolchain {
    pub type_: String,
    pub provides: Provides,
    pub configuration: HashMap<String, String>,
    span: Range<usize>,
    jdk_home_span: Option<Range<usize>>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Provides {
    pub version: String,
    pub vendor: Option<String>,
}

const EMPTY_TOOLCHAINS: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<toolchains>\n</toolchains>\n";

impl Toolchains {
    pub fn load() -> Self {
        let file = m2_dir().join("toolchains.xml");
        let content = if file.exists() {
            fs::read_to_string(&file).unwrap()
        } else {
            String::new()
        };
        Self::parse(content).unwrap_or_else(|e| {
            eprintln!("Failed to parse {}: {}", file.display(), e);
            std::process::exit(1);
        })
    }

    pub fn parse(content: String) -> Result<Self, String> {
        let content = if content.trim().is_empty() {
            EMPTY_TOOLCHAINS.to_string()
        } else {
            content
        };
        let mut reader = Reader::from_str(&content);
        let mut stack: Vec<String> = Vec::new();
        let mut toolchains = Vec::new();
        let mut root_end = None;
        let mut current: Option<Toolchain> = None;
        // content start of the current leaf element
        let mut text_start = 0usize;
        loop {
            let pos = reader.buffer_position() as usize;
            let event = reader.read_event().map_err(|e| format!("{} at position {}", e, reader.error_position()))?;
            match event {
                Event::Start(e) => {
                    let name = AsRef::<str>::as_ref(&e.local_name()).to_string();
                    if stack.len() == 1 && name == "toolchain" {
                        current = Some(Toolchain::empty(pos));
                    }
                    stack.push(name);
                    text_start = reader.buffer_position() as usize;
                }
                Event::Empty(e) => {
                    let name = AsRef::<str>::as_ref(&e.local_name()).to_string();
                    if stack.is_empty() && name == "toolchains" {
                        root_end = Some(RootEnd::Empty(pos..reader.buffer_position() as usize));
                    } else if stack.len() == 1 && name == "toolchain" {
                        let mut toolchain = Toolchain::empty(pos);
                        toolchain.span.end = reader.buffer_position() as usize;
                        toolchains.push(toolchain);
                    }
                }
                Event::End(_) => {
                    let end = reader.buffer_position() as usize;
                    let name = stack.pop().unwrap_or_default();
                    let path: Vec<&str> = stack.iter().skip(2).map(|s| s.as_str()).collect();
                    if stack.is_empty() && name == "toolchains" {
                        root_end = Some(RootEnd::Close(pos));
                    } else if stack.len() == 1 && name == "toolchain" {
                        if let Some(mut toolchain) = current.take() {
                            toolchain.span.end = end;
                            toolchains.push(toolchain);
                        }
                    } else if let Some(toolchain) = current.as_mut() {
                        let range = text_start..pos;
                        let value = leaf_text(&content[range.clone()]);
                        match (path.as_slice(), name.as_str()) {
                            ([], "type") => toolchain.type_ = value,
                            (["provides"], "version") => toolchain.provides.version = value,
                            (["provides"], "vendor") => toolchain.provides.vendor = Some(value),
                            (["configuration"], key) => {
                                if key == "jdkHome" {
                                    toolchain.jdk_home_span = Some(range);
                                }
                                toolchain.configuration.insert(key.to_string(), value);
                            }
                            _ => {}
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
        if root_end.is_none() {
            return Err("root element <toolchains> not found".to_string());
        }
        Ok(Toolchains {
            content,
            toolchain: toolchains,
            root_end,
        })
    }

    fn find_jdk(&self, version: &str, vendor: &Option<String>) -> Option<usize> {
        let vendor = normalize_vendor(vendor);
        self.toolchain.iter().position(|t| {
            t.type_ == "jdk" && t.provides.version == version && normalize_vendor(&t.provides.vendor) == vendor
        })
    }

    /// Add or update a JDK toolchain, returns false if the same entry already exists.
    pub fn add_jdk(&mut self, version: &str, vendor: Option<String>, jdk_home: String) -> bool {
        let (range, replacement) = if let Some(index) = self.find_jdk(version, &vendor) {
            let toolchain = &self.toolchain[index];
            if toolchain.configuration.get("jdkHome") == Some(&jdk_home) {
                return false;
            }
            match &toolchain.jdk_home_span {
                Some(span) => (span.clone(), escape(&jdk_home)),
                None => (toolchain.span.clone(), jdk_toolchain_xml(version, &vendor, &jdk_home, "  ")),
            }
        } else {
            match self.root_end.as_ref().unwrap() {
                RootEnd::Close(pos) => {
                    let line_start = self.content[..*pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    let block = format!("  {}\n", jdk_toolchain_xml(version, &vendor, &jdk_home, "  "));
                    if self.content[line_start..*pos].trim().is_empty() {
                        (line_start..line_start, block)
                    } else {
                        (*pos..*pos, format!("\n{}", block))
                    }
                }
                RootEnd::Empty(span) => {
                    let tag = &self.content[span.clone()];
                    let open_tag = format!("{}>", tag.trim_end_matches("/>").trim_end());
                    let block = jdk_toolchain_xml(version, &vendor, &jdk_home, "  ");
                    (span.clone(), format!("{}\n  {}\n</toolchains>", open_tag, block))
                }
            }
        };
        self.replace(range, &replacement);
        true
    }

    /// Remove a JDK toolchain, returns false if not found.
    pub fn remove_jdk(&mut self, version: &str, vendor: Option<String>) -> bool {
        let Some(index) = self.find_jdk(version, &vendor) else {
            return false;
        };
        let mut range = self.toolchain[index].span.clone();
        // remove the whole line if the element occupies it alone
        let line_start = self.content[..range.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = self.content[range.end..]
            .find('\n')
            .map(|i| range.end + i + 1)
            .unwrap_or(self.content.len());
        if self.content[line_start..range.start].trim().is_empty() && self.content[range.end..line_end].trim().is_empty() {
            range = line_start..line_end;
        }
        self.replace(range, "");
        true
    }

    fn replace(&mut self, range: Range<usize>, replacement: &str) {
        let mut content = std::mem::take(&mut self.content);
        content.replace_range(range, replacement);
        *self = Self::parse(content).expect("toolchains.xml became invalid after modification");
    }

    pub fn write(&self) {
        let file = m2_dir().join("toolchains.xml");
        fs::write(file, &self.content).unwrap();
    }
}

impl Toolchain {
    fn empty(start: usize) -> Self {
        Toolchain {
            type_: String::new(),
            provides: Provides::default(),
            configuration: HashMap::new(),
            span: start..start,
            jdk_home_span: None,
        }
    }
}

fn normalize_vendor(vendor: &Option<String>) -> Option<&str> {
    vendor.as_deref().map(str::trim).filter(|v| !v.is_empty())
}

fn escape(text: &str) -> String {
    quick_xml::escape::escape(text).to_string()
}

/// text of a leaf element: unescape entities and unwrap CDATA
fn leaf_text(raw: &str) -> String {
    let trimmed = raw.trim();
    if let Some(inner) = trimmed.strip_prefix("<![CDATA[").and_then(|s| s.strip_suffix("]]>")) {
        return inner.to_string();
    }
    quick_xml::escape::unescape(trimmed)
        .map(|s| s.to_string())
        .unwrap_or_else(|_| trimmed.to_string())
}

fn jdk_toolchain_xml(version: &str, vendor: &Option<String>, jdk_home: &str, indent: &str) -> String {
    let i2 = indent.repeat(2);
    let i3 = indent.repeat(3);
    let vendor_xml = match normalize_vendor(vendor) {
        Some(vendor) => format!("{i3}<vendor>{}</vendor>\n", escape(vendor)),
        None => String::new(),
    };
    format!(
        "<toolchain>\n{i2}<type>jdk</type>\n{i2}<provides>\n{i3}<version>{}</version>\n{vendor_xml}{i2}</provides>\n{i2}<configuration>\n{i3}<jdkHome>{}</jdkHome>\n{i2}</configuration>\n{indent}</toolchain>",
        escape(version),
        escape(jdk_home),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<toolchains xmlns="http://maven.apache.org/TOOLCHAINS/1.1.0">
  <!-- maintained by hand -->
  <toolchain>
    <type>jdk</type>
    <provides>
      <version>17</version>
      <vendor>temurin</vendor>
      <id>my-jdk17</id>
    </provides>
    <configuration>
      <jdkHome>/opt/jdk&amp;17</jdkHome>
    </configuration>
  </toolchain>
</toolchains>
"#;

    #[test]
    fn test_parse_toolchains() {
        let toolchains = Toolchains::parse(SAMPLE.to_string()).unwrap();
        assert_eq!(toolchains.toolchain.len(), 1);
        let t = &toolchains.toolchain[0];
        assert_eq!(t.type_, "jdk");
        assert_eq!(t.provides.version, "17");
        assert_eq!(t.provides.vendor.as_deref(), Some("temurin"));
        assert_eq!(t.configuration.get("jdkHome").unwrap(), "/opt/jdk&17");
    }

    #[test]
    fn test_add_jdk_no_duplicate() {
        let mut toolchains = Toolchains::parse(SAMPLE.to_string()).unwrap();
        assert!(toolchains.add_jdk("21", None, "/opt/jdk21".to_string()));
        assert!(!toolchains.add_jdk("21", None, "/opt/jdk21".to_string()));
        assert!(!toolchains.add_jdk("21", Some("".to_string()), "/opt/jdk21".to_string()));
        assert_eq!(toolchains.toolchain.len(), 2);
        // unknown fields and namespace are kept
        assert!(toolchains.content.contains("xmlns=\"http://maven.apache.org/TOOLCHAINS/1.1.0\""));
        assert!(toolchains.content.contains("<id>my-jdk17</id>"));
        assert!(toolchains.content.contains("<!-- maintained by hand -->"));
        assert!(toolchains.content.ends_with("  </toolchain>\n</toolchains>\n"));
    }

    #[test]
    fn test_update_jdk_home() {
        let mut toolchains = Toolchains::parse(SAMPLE.to_string()).unwrap();
        assert!(toolchains.add_jdk("17", Some("temurin".to_string()), "/opt/new-jdk17".to_string()));
        assert_eq!(toolchains.toolchain.len(), 1);
        assert!(toolchains.content.contains("<jdkHome>/opt/new-jdk17</jdkHome>"));
        assert!(toolchains.content.contains("<id>my-jdk17</id>"));
    }

    #[test]
    fn test_remove_jdk() {
        let mut toolchains = Toolchains::parse(SAMPLE.to_string()).unwrap();
        toolchains.add_jdk("21", None, "/opt/jdk21".to_string());
        assert!(toolchains.remove_jdk("21", None));
        assert_eq!(toolchains.content, SAMPLE);
        assert!(!toolchains.remove_jdk("21", None));
    }

    #[test]
    fn test_empty_toolchains() {
        let mut toolchains = Toolchains::parse("<toolchains/>".to_string()).unwrap();
        assert!(toolchains.toolchain.is_empty());
        toolchains.add_jdk("21", None, "/opt/jdk21".to_string());
        assert_eq!(toolchains.toolchain.len(), 1);
        let mut toolchains = Toolchains::parse(String::new()).unwrap();
        toolchains.add_jdk("21", None, "/opt/jdk21".to_string());
        assert_eq!(toolchains.toolchain.len(), 1);
    }
}

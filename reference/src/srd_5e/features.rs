use super::{write_text_block, Reference};
use serde::Deserialize;  

#[derive(Debug, Deserialize)]
pub struct Feature {
    index: String,
    pub name: String,
    class: Reference,
    subclass: Option<Reference>,

    level: u8,
    desc: Vec<String>,

    #[serde(default)]
    parent: Option<Reference>,


}

pub struct SummaryView<'a>(&'a Feature);

pub struct DetailsView<'a>(&'a Feature);

impl Feature {
    pub fn token(&self) -> String {
        crate::to_camel_case(&self.index)
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let class_feature = self.0;
        write!(f, "`{}`", class_feature.name)
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let class_feature = self.0;

        writeln!(f, "# {}", class_feature.name)?;

        write!(f, "\n\n**Class:** {}", class_feature.class.name)?;

        if let Some(subclass) = &class_feature.subclass {
            write!(f, "\n**Subclass:** {}", subclass.name)?;
        }

        write!(f, "\\\n**Level:** {}", class_feature.level)?;

        if !class_feature.desc.is_empty() {
            write!(f, "\n\n")?;
            write_text_block(f, &class_feature.desc[..])?;
        }

        Ok(())
    }
}
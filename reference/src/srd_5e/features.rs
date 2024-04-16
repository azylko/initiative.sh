use super::{write_text_block, Reference};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Feature {
    index: String,
    pub name: String,

    class: Reference,
    subclass: Option<Reference>,

    level: u8,
    desc: Vec<String>,

    feature_specific: Option<FeatureSpecific>,


    #[serde(default)]
    parent: Option<Reference>,


}

#[derive(Debug, Deserialize)]
pub struct FeatureSpecific {
    expertise_options: Option<ExpertiseOption>,
    subfeature_options: Option<SubfeatureOption>,
}

#[derive(Debug, Deserialize)]
pub struct ExpertiseOption {
    choose: u8,
    #[serde(rename = "type")]
    feature_type: String,

    from: Vec<Reference>
}

#[derive(Debug, Deserialize)]
pub struct SubfeatureOption {
    choose: u8,
    #[serde(rename = "type")]
    feature_type: String,

    from: Vec<Reference>
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

        if let Some(feature_specific) = &class_feature.feature_specific {
            if let Some(expertise_options) = &feature_specific.expertise_options {
                // Unwrap the Option to access the ExpertiseOption struct
                write!(f, "\n\nChoose {} {}", expertise_options.choose, expertise_options.feature_type)?;
                for option in &expertise_options.from {
                    write!(f, "\n- {}", option.name)?;
                }
            }

            if let Some(subfeature_options) = &feature_specific.subfeature_options {
                // Unwrap the Option to access the ExpertiseOption struct
                write!(f, "\n\nChoose {} {}", subfeature_options.choose, subfeature_options.feature_type)?;
                for option in &subfeature_options.from {
                    write!(f, "\n- {}", option.name)?;
                }
            }
        }

        if !class_feature.desc.is_empty() {
            write!(f, "\n\n")?;
            write_text_block(f, &class_feature.desc[..])?;
        }

        Ok(())
    }
}
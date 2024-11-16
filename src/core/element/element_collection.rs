use super::element::Element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(from = "serde_yaml::Sequence")]
#[serde(into = "serde_yaml::Sequence")]
pub struct ElementCollection {
    pub elements: Vec<Element>,
}

impl Into<serde_yaml::Sequence> for ElementCollection {
    fn into(self) -> serde_yaml::Sequence {
        let mut seq = serde_yaml::Sequence::new();

        for entry in self.elements.iter() {
            seq.push(serde_yaml::to_value(entry).unwrap());
        }

        seq
    }
}

impl From<serde_yaml::Sequence> for ElementCollection {
    fn from(value: serde_yaml::Sequence) -> Self {
        let mut result = ElementCollection {
            elements: Vec::new(),
        };

        for entry in value.iter() {
            let element = serde_yaml::from_value::<Element>(entry.clone()).unwrap();
            result.elements.push(element);
        }

        result
    }
}

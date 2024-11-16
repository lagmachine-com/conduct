use enum_dispatch::enum_dispatch;
use log::warn;
use serde::{Deserialize, Serialize};
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Value,
};

use super::operators::ElementOperator;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
#[serde(from = "serde_yaml::Value")]
#[serde(into = "serde_yaml::Value")]
pub enum Element {
    Value(String),
    Operator(ElementOperator),
}

impl Into<serde_yaml::Value> for Element {
    fn into(self) -> serde_yaml::Value {
        match self {
            Element::Value(value) => serde_yaml::Value::String(value),
            Element::Operator(element_operator) => {
                let result = serde_yaml::to_value(element_operator).unwrap();
                let map = result.as_mapping().unwrap();

                let result = match result {
                    Value::Tagged(ref tagged_value) => tagged_value,
                    _ => panic!(),
                };

                let mut tag = result.tag.to_string();

                let mut index: u32 = 0;
                let mut args = Vec::<String>::new();
                while let Some(result) = map.get(Value::String(index.to_string())) {
                    index += 1;
                    args.push(result.as_str().unwrap().to_string());
                }

                if args.is_empty() == false {
                    tag = format!("{}({})", tag, args.join(";"));
                }

                let value = TaggedValue {
                    tag: Tag::new(tag),
                    value: map.get(Value::String("value".to_string())).unwrap().clone(),
                };

                Value::Tagged(Box::new(value))
            }
        }
    }
}

impl From<serde_yaml::Value> for Element {
    fn from(value: serde_yaml::Value) -> Self {
        match value {
            serde_yaml::Value::String(str) => Element::Value(str.clone()),
            serde_yaml::Value::Tagged(tagged_value) => {
                let mut tag = tagged_value.tag.to_string();
                let value = &tagged_value.value;

                let mut map = Mapping::new();

                // Parse `operator(arg;arg2) value` syntax
                match tag.clone().split_once("(") {
                    Some(pargs) => {
                        let mut args_str = pargs.1;
                        tag = pargs.0.to_string();

                        args_str = args_str.split(")").next().unwrap();
                        for (i, arg) in args_str.split(";").into_iter().enumerate() {
                            map.insert(
                                Value::String(i.to_string()),
                                Value::String(arg.to_string()),
                            );
                        }
                    }
                    None => (),
                }

                map.insert(
                    serde_yaml::Value::String("value".to_string()),
                    value.clone(),
                );

                let value = TaggedValue {
                    tag: Tag::new(tag),
                    value: Value::Mapping(map),
                };

                let element = Element::Operator(
                    serde_yaml::from_value::<ElementOperator>(serde_yaml::Value::Tagged(Box::new(
                        value,
                    )))
                    .unwrap(),
                );

                element
            }
            _ => {
                warn!("Got unexpected value: {:?}", value);
                panic!()
            }
        }
    }
}

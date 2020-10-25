mod flusher;

pub use flusher::{normalize_string, NormalizeConfig};

use crate::*;
use seed::{prelude::*, *};

pub struct InputField {
    pub current_value: String,
    pub accepted_values: Vec<String>,
    pub correct_value: String,
    pub status: FieldStatus,
}

pub enum FieldStatus {
    Valid,
    Error,
    None,
}

impl std::fmt::Display for FieldStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valid => write!(f, "is-valid"),
            Self::Error => write!(f, "is-invalid"),
            Self::None => write!(f, ""),
        }
    }
}

impl InputField {
    pub fn new(config: &NormalizeConfig, from: Vec<impl Into<String> + Clone>) -> Self {
        let correct_value: String = from.get(0).unwrap().clone().into();
        Self {
            current_value: String::new(),
            accepted_values: from
                .into_iter()
                .map(|x| {
                    let x: String = x.into();
                    flusher::normalize_string(&x, config)
                })
                .collect(),
            correct_value,
            status: FieldStatus::None,
        }
    }
}

pub struct Entry {
    pub interogative: InputField,
    pub negative: InputField,
    pub question: Vec<Node<Msg>>,
}

pub enum Parts {
    Normal(String),
    Underlined(String),
    Concat(Box<Parts>, Box<Parts>),
}

impl std::str::FromStr for Parts {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("__") {
            let mut iter = s.split("__");

            let mut current = Self::Normal(iter.next().unwrap().to_owned());

            for (x, i) in iter.enumerate() {
                current = Self::Concat(
                    Box::new(current),
                    Box::new(if x % 2 == 0 {
                        Self::Underlined(i.to_owned())
                    } else {
                        Self::Normal(i.to_owned())
                    }),
                )
            }

            Ok(current)
        } else {
            Ok(Self::Normal(s.to_owned()))
        }
    }
}

impl Into<Vec<Node<Msg>>> for Parts {
    fn into(self) -> Vec<Node<Msg>> {
        match self {
            Parts::Normal(text) => vec![plain![text]],
            Parts::Concat(a1, a2) => {
                let mut vec: Vec<Node<Msg>> = Parts::into(*a1);
                let vec1: Vec<Node<Msg>> = Parts::into(*a2);
                vec.extend(vec1.into_iter());
                vec
            }
            Parts::Underlined(text) => vec![u![text]],
        }
    }
}

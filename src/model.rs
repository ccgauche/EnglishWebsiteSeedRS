use crate::app::{Entry, InputField, NormalizeConfig, Parts};

pub struct Model {
    pub points: u32,
    pub tries: u32,
    pub current_question: Entry,
    pub config: NormalizeConfig,
}

impl<'a> Default for Model {
    fn default() -> Self {
        let config: NormalizeConfig = NormalizeConfig::default();
        Self {
            points: 0,
            tries: 0,
            current_question: Entry {
                interogative: InputField::new(
                    &config,
                    vec!["Do you like cats ?", "Do you love cats ?"],
                ),
                negative: InputField::new(&config, vec!["I don't like cats."]),
                question: "I __like cats__".parse::<Parts>().unwrap().into(),
            },
            config,
        }
    }
}
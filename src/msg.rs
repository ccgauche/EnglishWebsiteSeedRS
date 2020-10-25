use crate::Model;

use seed::prelude::Orders;

#[derive(Clone, Eq, PartialEq)]
pub enum Msg {
    None,
    Answer,
    Valid,
    UpdateInterogation(String),
    UpdateNegative(String),
}

impl Msg {
    pub fn update(self, model: &mut Model, _: &mut impl Orders<Msg>) {
        match self {
            Self::None => (),
            Self::Answer => {}
            Self::Valid => {
                {
                    let out = crate::app::normalize_string(
                        &model.current_question.interogative.current_value,
                        &model.config,
                    );
                    if model
                        .current_question
                        .interogative
                        .accepted_values
                        .contains(&out)
                    {
                        model.current_question.interogative.status = crate::FieldStatus::Valid;
                        model.points += 3;
                    } else {
                        model.current_question.interogative.status = crate::FieldStatus::Error;
                        model.points += 2;
                    }
                }
                {
                    let out = crate::app::normalize_string(
                        &model.current_question.negative.current_value,
                        &model.config,
                    );
                    if model
                        .current_question
                        .negative
                        .accepted_values
                        .contains(&out)
                    {
                        model.current_question.negative.status = crate::FieldStatus::Valid;
                        model.points += 3;
                    } else {
                        model.current_question.negative.status = crate::FieldStatus::Error;
                        model.points += 2;
                    }
                }
            }
            Self::UpdateInterogation(text) => {
                model.current_question.interogative.current_value = text;
            }
            Self::UpdateNegative(text) => {
                model.current_question.negative.current_value = text;
            }
        }
    }
}

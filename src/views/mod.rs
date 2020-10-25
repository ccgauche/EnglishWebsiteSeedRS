use crate::*;
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![navbar(model), container(model), footer()]
}

fn footer() -> Node<Msg> {
    footer![
        C![
            "page-footer",
            "font-small",
            "bg-dark",
            "fixed-bottom",
            "text-light"
        ],
        div![
            C!["footer-copyright", "text-center", "py-3"],
            "Made with ♥ by ",
            a![
                attrs! {
                    At::Href => "https://github.com/ccgauche"
                },
                "Laurent Gaucheron"
            ]
        ]
    ]
}

fn container(model: &Model) -> Node<Msg> {
    div![
        C!["container", "mt-4"],
        h2![
            C!["pb-4", "pt-2", "text-center"],
            &model.current_question.question
        ],
        div![
            C!["form-group", "row", "mt-4"],
            div![
                C!["form-group", "col-lg-12"],
                build_field(
                    "Interogative form",
                    &model.current_question.interogative.status,
                    Msg::UpdateInterogation
                ),
                build_field(
                    "Negative form",
                    &model.current_question.negative.status,
                    Msg::UpdateNegative
                )
            ]
        ],
        div![
            C!["col", "text-center", "mt-2"],
            style! {
                "margin-bottom" => "5rem!important"
            },
            button![
                C![
                    "btn",
                    "btn-secondary",
                    "col-lg-2",
                    "col-md-5",
                    "col-sm-5",
                    "mt-2",
                    "mr-1"
                ],
                "Answers"
            ],
            button![
                C![
                    "btn",
                    "btn-primary",
                    "col-lg-2",
                    "col-md-5",
                    "col-sm-5",
                    "mt-2",
                    "ml-1"
                ],
                ev(Ev::Click, |_| Msg::Valid),
                "Check"
            ]
        ]
    ]
}

fn build_field(
    name: &str,
    status: &FieldStatus,
    handler: impl FnOnce(String) -> Msg + 'static + Clone,
) -> Node<Msg> {
    div![
        C!["form-group", "row"],
        label![C!["col-lg-2", "col-form-label"], name],
        div![
            C!["col-lg-10"],
            input![
                C!["form-control", status.to_string()],
                input_ev(Ev::Input, handler),
                attrs! {
                    "autocomplete" => "off",
                    "autocorrect" => "off",
                    "autocapitalize" => "off",
                    "spellcheck" => "false",
                }
            ]
        ]
    ]
}

#[inline]
fn navbar(model: &Model) -> Node<Msg> {
    nav![
        C!["navbar", "navbar-expand-lg", "navbar-dark", "bg-primary"],
        a![
            C!["navbar-brand", "mr-auto"],
            "Sentence gymnastics training"
        ],
        ul![
            C!["navbar-nav"],
            li![
                C!["nav-item", "active"],
                a![C!["nav-link"], model.points, " 🟊"]
            ]
        ]
    ]
}

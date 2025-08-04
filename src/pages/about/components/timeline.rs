use leptos::prelude::*;
use strum_macros::EnumIter;

use crate::pages::about::pages::{
    about::page::AboutMe,
    appendix::page::Appendix,
    intro::page::Intro,
    skills::page::Skills,
    timeline::{
        twenty_five::TwentyTwentyFive, twenty_four::TwentyTwentyFour,
        twenty_three::TwentyTwentyThree, twenty_two::TwentyTwentyTwo,
    },
};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Timeline {
    #[default]
    Intro,
    About,
    Skills,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    Appendix,
}

impl Timeline {
    pub fn into_string(&self) -> String {
        match self {
            Self::Intro => String::from("Introduction"),
            Self::About => String::from("About Me"),
            Self::Skills => String::from("Skills"),
            Self::TwentyTwo => String::from("2022"),
            Self::TwentyThree => String::from("2023"),
            Self::TwentyFour => String::from("2024"),
            Self::TwentyFive => String::from("2025"),
            Self::Appendix => String::from("Appendix"),
        }
    }

    pub fn render(&self) -> impl IntoView {
        match self {
            Self::Intro => view! {<Intro/>}.into_any(),
            Self::About => view! {<AboutMe/>}.into_any(),
            Self::Skills => view! {<Skills/>}.into_any(),
            Self::TwentyTwo => view! {<TwentyTwentyTwo/>}.into_any(),
            Self::TwentyThree => view! {<TwentyTwentyThree/>}.into_any(),
            Self::TwentyFour => view! {<TwentyTwentyFour/>}.into_any(),
            Self::TwentyFive => view! {<TwentyTwentyFive/>}.into_any(),
            Self::Appendix => view! {<Appendix/>}.into_any(),
        }
    }
}

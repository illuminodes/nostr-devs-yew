use yew::prelude::*;

use crate::{AppLink, AppRoute, Card, Heading, Text, TranslationStore, MEETUP_HISTORY};

impl Default for Heading {
    fn default() -> Self {
        Self {
            size: crate::HeadingSize::H1,
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            hover: crate::CardHover::Default,
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            variant: crate::TextVariant::Default,
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct HomeProps {
    #[prop_or_default]
    pub heading: Heading,
    #[prop_or_default]
    pub card: Card,
    #[prop_or_default]
    pub text: Text,
}
#[function_component(HomePage)]
pub fn home_page(props: &HomeProps) -> Html {
    let HomeProps {
        heading,
        card,
        text,
    } = props;
    let translations = use_context::<TranslationStore>().expect("Translation Context not found");
    html! {
        <div class="gap-4 sm:gap-8 h-full flex flex-col">
            <h1 class={heading.to_class()}>{translations.find_translation("meetup_list_title")}</h1>
            <ul class="space-y-4">
                {MEETUP_HISTORY.iter().map(|meetup| {
                    html! {
                        <li
                            key={meetup.id}
                            class={card.to_class()}
                        >
                            <AppLink route={AppRoute::Meetup { id: meetup.id.to_string() }}
                                    classes="flex justify-between items-center w-full">
                                    <span class="font-semibold text-gray-100">{meetup.title}</span>
                                    <span class={text.to_class()}>{meetup.date}</span>
                            </AppLink>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

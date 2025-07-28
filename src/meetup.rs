use lucide_yew::Earth;
use yew::prelude::*;

use crate::{Heading, HeadingSize, List, Text, TranslationStore, MEETUP_HISTORY};

impl Default for List {
    fn default() -> Self {
        Self {
            variant: crate::ListVariant::Items,
        }
    }
}
#[derive(PartialEq, Properties)]
pub struct MeetupPageProps {
    pub id: String,
    #[prop_or_default]
    pub heading: Heading,
    #[prop_or_default]
    pub text: Text,
    #[prop_or_default]
    pub list: List,
}
#[function_component(MeetupPage)]
pub fn meetup_page(props: &MeetupPageProps) -> Html {
    let MeetupPageProps {
        id: meet_id,
        heading,
        text,
        list,
    } = props;
    let subheading = Heading {
        size: HeadingSize::H3,
    };
    let translator = use_context::<TranslationStore>().expect("Translation Context not found");
    let Some(meetup) = MEETUP_HISTORY.iter().find(|meetup| meetup.id == meet_id) else {
        return html! {<p class={text.to_class()}>{"Meetup not found"}</p>};
    };

    html! {
        <div class="gap-4 sm:gap-8 h-full flex flex-col">
            <h1 class={heading.to_class()}>{meetup.title}</h1>
            <div class="px-4">
                <p class={subheading.to_class()}>{meetup.date}</p>
            </div>

            <h2 class={subheading.to_class()}>{translator.find_translation("topics_title")}</h2>
            <ul class={list.to_class()}>
                {meetup.topics.iter().map(|topic| {
                    html! {
                        <li class={"flex flex-row items-center justify-between"} >
                            <p class={text.to_class()}>
                                {topic.0}
                            </p>
                            <a  href={topic.1} target="_blank" rel="noopener noreferrer" >
                                <Earth class="w-4 h-4 sm:w-6 sm:h-6 md:w-8 md:h-8 lg:w-10 lg:h-10 xl:w-12 xl:h-12
                                stroke-gray-300" />
                            </a>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>

            <h2 class={subheading.to_class()}>{translator.find_translation("housekeeping_title")}</h2>
            <ul class={list.to_class()}>
                <li class={Text { variant: crate::TextVariant::Small }.to_class()} >{translator.find_translation("housekeeping_list_item_1")}</li>
                <li class={Text { variant: crate::TextVariant::Small }.to_class()} >{translator.find_translation("housekeeping_list_item_2")}</li>
                <li class={Text { variant: crate::TextVariant::Small }.to_class()} >{translator.find_translation("housekeeping_list_item_3")}</li>
                <li class={Text { variant: crate::TextVariant::Small }.to_class()} >{translator.find_translation("housekeeping_list_item_4")}</li>
                <li class={Text { variant: crate::TextVariant::Small }.to_class()} >{translator.find_translation("housekeeping_list_item_5")}</li>
            </ul>

        </div>
    }
}

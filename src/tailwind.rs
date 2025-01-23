use tailwind_fuse::*;

#[derive(TwClass, Default, PartialEq)]
#[tw(class = "space-y-4")]
pub struct List {
    pub variant: ListVariant,
}
#[derive(TwVariant, PartialEq)]
pub enum ListVariant {
    #[tw(default, class = "list-disc list-inside mb-4 px-4 overflow-y-auto max-h-1/3")]
    Items,
    #[tw(class = "")]
    Normal,
}

#[derive(TwClass, PartialEq, Default)]
#[tw(
    class = "border p-4 rounded-lg transition-colors duration-200 max-w-md sm:max-w-xl md:max-w-4xl"
)]
pub struct Card {
    pub hover: CardHover,
}
#[derive(TwVariant, PartialEq)]
pub enum CardHover {
    #[tw(default, class = "hover:bg-nostr-light")]
    Default,
    #[tw(class = "hover:bg-gray-100")]
    Light,
}

#[derive(TwClass, Default, PartialEq)]
#[tw(class = "font-bold text-gray-100")]
pub struct Heading {
    pub size: HeadingSize,
}
#[derive(TwVariant, PartialEq)]
pub enum HeadingSize {
    #[tw(default, class = "text-3xl sm:text-4xl lg:text-5xl")]
    H1,
    #[tw(class = "text-2xl sm:text-3xl lg:text-4xl")]
    H2,
    #[tw(class = "text-xl mb-2")]
    H3,
    #[tw(class = "text-3xl sm:text-4xl lg:text-5xl")]
    H1NoMargin,
    #[tw(class = "text-2xl sm:text-3xl lg:text-4xl")]
    H2NoMargin,
    #[tw(class = "text-xl sm:text-2xl lg:text-3xl")]
    H3NoMargin,
}

#[derive(TwClass, PartialEq, Default)]
#[tw(class = "text-gray-300")]
pub struct Text {
    pub variant: TextVariant,
}
#[derive(TwVariant, PartialEq)]
pub enum TextVariant {
    #[tw(default, class = "text-base sm:text-lg md:text-xl lg:text-2xl")]
    Default,
    #[tw(class = "text-sm")]
    Small,
    #[tw(class = "text-lg sm:text-xl md:text-2xl lg:text-3xl")]
    Large,
}

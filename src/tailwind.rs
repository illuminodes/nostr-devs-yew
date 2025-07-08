#[derive(PartialEq)]
pub struct List { pub variant: ListVariant }
#[derive(PartialEq)]
pub enum ListVariant { Items, Normal }

impl List {
    pub fn to_class(&self) -> String {
        let base = "space-y-4";
        let variant_cls = match self.variant {
            ListVariant::Items => "list-disc list-inside mb-4 px-4 overflow-y-auto max-h-1/3",
            ListVariant::Normal => "",
        };
        format!("{} {}", base, variant_cls)
    }
}

#[derive(PartialEq)]
pub struct Card { pub hover: CardHover }
#[derive(PartialEq)]
pub enum CardHover { Default, Light }

impl Card {
    pub fn to_class(&self) -> String {
        let base = "border p-4 rounded-lg transition-colors duration-200 max-w-md sm:max-w-xl md:max-w-4xl";
        let cls = match self.hover {
            CardHover::Default => "hover:bg-nostr-light",
            CardHover::Light => "hover:bg-gray-100",
        };
        format!("{} {}", base, cls)
    }
}

#[derive(PartialEq)]
pub struct Heading {
    pub size: HeadingSize,
}

#[derive(PartialEq)]
pub enum HeadingSize {
    H1,
    H2,
    H3,
    H1NoMargin,
    H2NoMargin,
    H3NoMargin,
}

impl Heading {
    pub fn to_class(&self) -> String {
        let base = "font-bold text-gray-100";
        let size_class = match self.size {
            HeadingSize::H1 => "text-3xl sm:text-4xl lg:text-5xl",
            HeadingSize::H2 => "text-2xl sm:text-3xl lg:text-4xl",
            HeadingSize::H3 => "text-xl mb-2",
            HeadingSize::H1NoMargin => "text-3xl sm:text-4xl lg:text-5xl",
            HeadingSize::H2NoMargin => "text-2xl sm:text-3xl lg:text-4xl",
            HeadingSize::H3NoMargin => "text-xl sm:text-2xl lg:text-3xl",
        };
        format!("{} {}", base, size_class)
    }
}

#[derive(PartialEq)]
pub struct Text {
    pub variant: TextVariant,
}
#[derive(PartialEq)]
pub enum TextVariant {
    Default,
    Small,
    Large,
}

impl Text {
    pub fn to_class(&self) -> String {
        let base = "text-gray-300";
        let variant_class = match self.variant {
            TextVariant::Default => "text-base sm:text-lg md:text-xl lg:text-2xl",
            TextVariant::Small => "text-sm",
            TextVariant::Large => "text-lg sm:text-xl md:text-2xl lg:text-3xl",
        };
        format!("{} {}", base, variant_class)
    }
}
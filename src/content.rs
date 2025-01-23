#[derive(Clone)]
pub struct MeetupDetails {
    pub id: &'static str,
    pub date: &'static str,
    pub title: &'static str,
    pub topics: &'static [(&'static str, &'static str)],
}
pub const MEETUP_HISTORY: [MeetupDetails; 2] = [
    MeetupDetails {
        id: "1",
        date: "2025-01-30",
        title: "Meetup Inaugural 2025",
        topics: &[
            ("Ian Carroll", "https://primal.net/e/note10gu7qwzsv9h7y03pt928q6s2k44egvwc8a050ljanteuv7tnmqps0fxgze"),
            ("Nostr + STARK", "https://njump.me/nevent1qqs9vsmtjf0uf22xwnahmxmly0t4g8vejr8lxym2h6zt990vme4770spzpmhxue69uhkummnw3ezumt0d5hsyg9c7npwjrcd6en3y9fn67ud4lth8p9sk5z3lqnju4ynck8hlylpfvpsgqqqqqqsv73x5z"),
            ("Kanbanstr", "https://www.kanbanstr.com"),
            ("SALUD Global Challenge", "https://www.kanbanstr.com/#/board/be7bf5de068c1d842ed34a7c270507ec940f5ea51671cfd062a95e9d09420d0a/e0d2c910-d0fc-421e-8e7f-ee00c1c2e219"),
            ("Illuminodes Update: Resin", "https://resin.estate/home"),
            ("Illuminodes Update: Fuente", "https://fuenteconsumer.theconstruct.work/"),
        ],
    },
    MeetupDetails {
        id: "0",
        date: "2024",
        title: "Resumen 2024",
        topics: &[
            (
                    "La NSA se une a Nostr", 
                    "https://njump.me/nevent1qqsvyh0e0a49gnesf3yek46fg5efzdv5tvevrmuvs4v696fsjmckjpqzyzslgkdxahcay09sp0fcr09r76zcfz7xryg808nph93uflkpayzfjhtuhq0"
            ),
            (
                "Gossip Model vs Distribucion Masiva",
                "https://mikedilger.com/gossip-model/",
            ),
            ("Fiatjaf desanonimizado!", "https://www.businessinsider.com/jack-dorsey-fiatjaf-nostr-donation-2024-6?op=1"),
            ("Servicios Ilegales de Cryptologia", "https://x.com/wikileaks/status/1828151621651447908"),
            ("Relevos WoT - Redes de Confianza", "https://github.com/bitvora/wot-relay/tree/master"),
            ("Lanzamiento de Pubky", "https://medium.com/@synonym_to/pubky-launch-260f36ba8fe3"),
            ("Relevos usando Tor", "https://njump.me/nevent1qqsd4vcrymtav7jt8jmyykv7utevq004gu4datyusvn8a7p47yhf7lsprpmhxue69uhkummnw3ezuendwsh8w6t69e3xj730qgs04xzt6ldm9qhs0ctw0t58kf4z57umjzmjg6jywu0seadwtqqc75srqsqqqqqpg48y67"),
            ("Un Pais construido sobre Open Source", "https://njump.me/nevent1qqs2axt6z58ta763qr8s2rrw2k2ymfzw8ayqqt5p5sy3pyux73trvpgprpmhxue69uhkummnw3ezuendwsh8w6t69e3xj730qgsrums8xku296t03nmx8ajdqjaee65nrt7v2l65ylp4hd59n62u3gsrqsqqqqqpqch0u7"),
        ],
    },
];

pub const LATEST_MEETUP_URL: &str = env!("LATEST_MEETUP_URL");
pub const GITHUB_URL: &str = env!("GH_URL");
pub const X_URL: &str = env!("X_URL");
pub const PRIMAL_URL: &str = env!("PRIMAL_URL");

use std::fmt::Display;

enum Env {
    GithubUrl(String),
    XUrl(String),
    PrimalUrl(String),
    LatestMeetupUrl(String),
}
impl TryFrom<&str> for Env {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (key, value) = value.split_once("=").ok_or("Invalid line in .env file")?;
        match key {
            "GH_URL" => Ok(Self::GithubUrl(value.to_string())),
            "X_URL" => Ok(Self::XUrl(value.to_string())),
            "PRIMAL_URL" => Ok(Self::PrimalUrl(value.to_string())),
            "LATEST_MEETUP_URL" => Ok(Self::LatestMeetupUrl(value.to_string())),
            _ => Err("Invalid key in .env file"),
        }
    }
}
impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GithubUrl(value) => write!(f, "{}", value),
            Self::XUrl(value) => write!(f, "{}", value),
            Self::PrimalUrl(value) => write!(f, "{}", value),
            Self::LatestMeetupUrl(value) => write!(f, "{}", value),
        }
    }
}

fn main() {
    let env_file = include_str!("./.env");
    let mut vars = vec![];
    for line in env_file.lines() {
        if let Ok(env) = Env::try_from(line) {
            vars.push(env);
        }
    }
    println!("cargo:rustc-env=GH_URL={}", vars[0]);
    println!("cargo:rustc-env=X_URL={}", vars[1]);
    println!("cargo:rustc-env=PRIMAL_URL={}", vars[2]);
    println!("cargo:rustc-env=LATEST_MEETUP_URL={}", vars[3]);
}

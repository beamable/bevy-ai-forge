/// Domain is one of “game” (backend) or “client” (Bevy).
/// Game stats can only be retrieved from microservices,
/// but client stats can be retrieved both in microservices and in game code
#[derive(Debug, Default, PartialEq, Eq)]
pub enum StatDomainType {
    #[default]
    Client,
    Game,
}

/// Public client stats can be retrieved by anyone who knows your ID.
/// Private client stats in client can only be retrieved for yourself.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum StatAccessType {
    #[default]
    Public,
    Private,
}

impl std::fmt::Display for StatAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            StatAccessType::Public => f.write_str("public"),
            StatAccessType::Private => f.write_str("private"),
        }
    }
}

impl std::fmt::Display for StatDomainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            StatDomainType::Client => f.write_str("client"),
            StatDomainType::Game => f.write_str("game"),
        }
    }
}

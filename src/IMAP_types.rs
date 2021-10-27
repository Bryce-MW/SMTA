use std::fmt::{Display, Formatter};

pub struct Command {
    pub tag: Tag,
    pub command: CommandType
}
pub enum Tag {
    Tagged(String),
    Untagged,
    // Continuation
}
impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Tagged(tag) => write!(f, "{}", tag),
            Tag::Untagged => write!(f, "*")
        }
    }
}
pub enum CommandType {
    Any(Any),
    // Auth(Auth),
    NoAuth(NoAuth),
    // Select(Select)
}
impl CommandType {
    pub fn parse(input: &str) -> Option<Self> {
        Any::parse(input).map(CommandType::Any)
            .or_else(|| NoAuth::parse(input).map(CommandType::NoAuth))
    }
}
pub enum Any {
    Capability,
    // Logout,
    // Noop,
    // X(String)
}
impl Any {
    fn parse(input: &str) -> Option<Self> {
        // TODO(bryce): Make this actually case insensitive
        match input {
            "CAPABILITY" => Some(Self::Capability),
            _ => None,
        }
    }
}
pub enum Auth {
    // Append(Append),
    // Create(Create),
    // Delete(Delete),
    // Examine(Examine),
    // List(List),
    // LSub(LSub),
    // Rename(Rename),
    // Select(Select),
    // Status(Status),
    // Subscribe(Subscribe),
    // Unsubscribe(Unsubscribe)
}
pub enum NoAuth {
    // Login(Login),
    // Authenticate(Authenticate),
    StartTLS
}
impl NoAuth {
    fn parse(input: &str) -> Option<Self> {
        // TODO(bryce): Make this actually case insensitive
        match input {
            "STARTTLS" => Some(Self::StartTLS),
            _ => None,
        }
    }
}
pub enum Select {
    // Check,
    // Close,
    // Expunge,
    // Copy(Copy),
    // Fetch(Fetch),
    // Store(Store),
    // Uid(Uid),
    // Search(Search)
}
pub enum ResponseState {
    Ok,
    No,
    Bad
}
impl Display for ResponseState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseState::Ok => write!(f, "OK"),
            ResponseState::No => write!(f, "NO"),
            ResponseState::Bad => write!(f, "BAD")
        }
    }
}

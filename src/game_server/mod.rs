//! Game server (clients can connect as players or spectators)
pub(crate) mod player;
pub(crate) mod server;
pub(crate) mod spectator;
pub(crate) mod team;

pub use player::Player;
pub use server::GameServer;
pub use spectator::Spectator;
pub use team::Team;

use std::{fmt, str::FromStr};

/// Identifier for a lobby.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct LobbyId(i64);

/// Join code bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct JoinCode([u8; 3]);

impl JoinCode {
    pub fn new(bytes: [u8; 3]) -> Self {
        Self(bytes)
    }
}

impl fmt::Display for JoinCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", base16ct::HexDisplay(&self.0))
    }
}

impl FromStr for JoinCode {
    type Err = base16ct::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0; 3];
        base16ct::mixed::decode(s, &mut buf)?;
        Ok(JoinCode(buf))
    }
}

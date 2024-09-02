use std::fmt;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use clap::ValueEnum;
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JwtAlgorithm(Algorithm);

impl FromStr for JwtAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "HS256" => Ok(JwtAlgorithm(Algorithm::HS256)),
            "HS384" => Ok(JwtAlgorithm(Algorithm::HS384)),
            "HS512" => Ok(JwtAlgorithm(Algorithm::HS384)),
            "RS256" => Ok(JwtAlgorithm(Algorithm::RS256)),
            "RS384" => Ok(JwtAlgorithm(Algorithm::RS384)),
            "RS512" => Ok(JwtAlgorithm(Algorithm::RS512)),
            "ES256" => Ok(JwtAlgorithm(Algorithm::ES256)),
            "ES384" => Ok(JwtAlgorithm(Algorithm::ES384)),
            "PS256" => Ok(JwtAlgorithm(Algorithm::PS256)),
            "PS384" => Ok(JwtAlgorithm(Algorithm::PS384)),
            "PS512" => Ok(JwtAlgorithm(Algorithm::PS512)),
            "EDDSA" => Ok(JwtAlgorithm(Algorithm::EdDSA)),
            _ => Err(format!("Unsupported algorithm: {}", s)),
        }
    }
}

impl From<JwtAlgorithm> for &'static str {
    fn from(algo: JwtAlgorithm) -> &'static str {
        match algo.0 {
            Algorithm::HS256 => "HS256",
            Algorithm::HS384 => "HS384",
            Algorithm::HS512 => "HS512",
            Algorithm::RS256 => "RS256",
            Algorithm::RS384 => "RS384",
            Algorithm::RS512 => "RS512",
            Algorithm::ES256 => "ES256",
            Algorithm::ES384 => "ES384",
            Algorithm::PS256 => "PS256",
            Algorithm::PS384 => "PS384",
            Algorithm::PS512 => "PS512",
            Algorithm::EdDSA => "EdDSA",
        }
    }
}

impl Display for JwtAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)  // 直接使用 Debug 表示算法名称
    }
}

/// 为 `Algorithm` 实现 `ValueEnum`，以便与 `clap` 集成
impl ValueEnum for JwtAlgorithm {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            JwtAlgorithm(Algorithm::HS256),
            JwtAlgorithm(Algorithm::HS384),
            JwtAlgorithm(Algorithm::HS512),
            JwtAlgorithm(Algorithm::RS256),
            JwtAlgorithm(Algorithm::RS384),
            JwtAlgorithm(Algorithm::RS512),
            JwtAlgorithm(Algorithm::ES256),
            JwtAlgorithm(Algorithm::ES384),
            JwtAlgorithm(Algorithm::PS256),
            JwtAlgorithm(Algorithm::PS384),
            JwtAlgorithm(Algorithm::PS512),
            JwtAlgorithm(Algorithm::EdDSA),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            Algorithm::HS256 => Some(clap::builder::PossibleValue::new("HS256")),
            Algorithm::HS384 => Some(clap::builder::PossibleValue::new("HS384")),
            Algorithm::HS512 => Some(clap::builder::PossibleValue::new("HS512")),
            Algorithm::RS256 => Some(clap::builder::PossibleValue::new("RS256")),
            Algorithm::RS384 => Some(clap::builder::PossibleValue::new("RS384")),
            Algorithm::RS512 => Some(clap::builder::PossibleValue::new("RS512")),
            Algorithm::ES256 => Some(clap::builder::PossibleValue::new("ES256")),
            Algorithm::ES384 => Some(clap::builder::PossibleValue::new("ES384")),
            Algorithm::PS256 => Some(clap::builder::PossibleValue::new("PS256")),
            Algorithm::PS384 => Some(clap::builder::PossibleValue::new("PS384")),
            Algorithm::PS512 => Some(clap::builder::PossibleValue::new("PS512")),
            Algorithm::EdDSA => Some(clap::builder::PossibleValue::new("EDDSA")),
        }
    }
}

// 实现 Deref trait
impl Deref for JwtAlgorithm {
    type Target = Algorithm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: u64,
}

impl Claims {
    pub fn new(sub: String, aud: String, exp: u64) -> Self {
        Self { sub, aud, exp }
    }
}

mod util;
mod jwt;
mod cli;

use std::io::Read;
use clap::{Parser};
use jsonwebtoken::{Algorithm, DecodingKey, encode, EncodingKey, get_current_timestamp, Header, Validation};
use crate::cli::{Cli, Command};
use crate::jwt::{Claims};
use crate::util::{parse_duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Sign(args) => {
            let algo = *args.algo;

            let exp = get_current_timestamp() + parse_duration(&args.exp)?.as_secs();

            let header = Header::new(algo);
            let claims = Claims::new(args.sub, args.aud, exp);

            let mut reader = std::fs::File::open(&args.secret)?;
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;

            let key = match algo {
                Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => &EncodingKey::from_secret(&buf),
                Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => &EncodingKey::from_rsa_pem(&buf)?,
                Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => &EncodingKey::from_rsa_pem(&buf)?,
                Algorithm::ES256 | Algorithm::ES384 => &EncodingKey::from_ec_pem(&buf)?,
                Algorithm::EdDSA => &EncodingKey::from_rsa_pem(&buf)?,
            };

            let token = encode(&header, &claims, key)?;

            println!("Generated: {:?}", token);
        }
        Command::Verify(args) => {
            let algo = *args.algo;

            let mut reader = std::fs::File::open(&args.secret)?;
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;

            let mut v = Validation::new(algo);
            v.validate_aud = false;

            let key = match algo {
                Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => &DecodingKey::from_secret(&buf),
                Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => &DecodingKey::from_rsa_pem(&buf)?,
                Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => &DecodingKey::from_rsa_pem(&buf)?,
                Algorithm::ES256 | Algorithm::ES384 => &DecodingKey::from_ec_pem(&buf)?,
                Algorithm::EdDSA => &DecodingKey::from_rsa_pem(&buf)?,
            };

            let decoded = jsonwebtoken::decode::<Claims>(&args.token, key, &v)?;

            println!("Decoded: {:?}", decoded.claims);
        }
    }

    Ok(())
}

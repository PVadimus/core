use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub redis: Database,
    pub postgres: Database,
    pub clickhouse: ClickhouseDatabase,
    pub fiat: Fiat,
    pub moonpay: MoonPay,
    pub transak: Transak,
    pub mercuryo: Mercuryo,
    pub ramp: Ramp,
    pub banxa: Banxa,
    pub kado: Kado,
    pub coingecko: CoinGecko,
    pub pricer: Pricer,
    pub charter: Charter,
    pub name: Name,
    pub metrics: Metrics,
    pub assets: Assets,
    pub chains: Chains,
    pub parser: Parser,
    pub daemon: Daemon,
    pub pusher: Pusher,
    pub swap: Swap,
    pub alerter: Alerter,
    pub security: Security,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Fiat {
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ClickhouseDatabase {
    pub url: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct KeyPublic {
    pub public: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct KeySecret {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Key {
    pub secret: String,
    pub public: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct MoonPay {
    pub key: Key,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Transak {
    pub key: KeyPublic,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Mercuryo {
    pub key: Key,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Ramp {
    pub key: KeyPublic,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Banxa {
    pub url: String,
    pub key: Key,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Kado {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct CoinGecko {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Pricer {
    pub timer: u64,
    pub outdated: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Charter {
    pub timer: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Name {
    pub ens: URL,
    pub ud: UD,
    pub sns: URL,
    pub ton: URL,
    pub eths: URL,
    pub spaceid: URL,
    pub did: URL,
    pub suins: URL,
    pub aptos: URL,
    pub injective: URL,
    pub icns: URL,
    pub lens: URL,
    pub base: URL,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct URL {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct UD {
    pub url: String,
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Metrics {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Assets {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Chains {
    pub solana: Chain,
    pub ethereum: Chain,
    pub smartchain: Chain,
    pub polygon: Chain,
    pub optimism: Chain,
    pub arbitrum: Chain,
    pub base: Chain,
    pub opbnb: Chain,
    pub avalanchec: Chain,
    pub ton: Chain,
    pub cosmos: Chain,
    pub osmosis: Chain,
    pub thorchain: Chain,
    pub celestia: Chain,
    pub tron: Chain,
    pub xrp: Chain,
    pub aptos: Chain,
    pub sui: Chain,
    pub bitcoin: Chain,
    pub litecoin: Chain,
    pub doge: Chain,
    pub fantom: Chain,
    pub gnosis: Chain,
    pub injective: Chain,
    pub sei: Chain,
    pub manta: Chain,
    pub blast: Chain,
    pub noble: Chain,
    pub zksync: Chain,
    pub linea: Chain,
    pub mantle: Chain,
    pub celo: Chain,
    pub near: Chain,
    pub world: Chain,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Chain {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Parser {
    pub timeout: u64,
    pub retry: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Daemon {
    pub service: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Pusher {
    pub url: String,
    pub ios: PusherIOS,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct PusherIOS {
    pub topic: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Swap {
    pub jupiter: SwapProvider,
    pub thorchain: SwapProvider,
    pub aftermath: SwapProvider,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Security {
    pub hashdit: SecurityProvider,
    pub goplus: SecurityProvider,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct SwapFee {
    pub percent: f64,
    pub address: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct SwapProvider {
    pub url: String,
    pub key: String,
    pub fee: SwapFee,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct SecurityProvider {
    pub url: String,
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Alerter {
    pub update_interval_seconds: u64,
    pub rules: AlerterRules,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AlerterRules {
    pub price_increase_percent: f64,
    pub price_decrease_percent: f64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let current_dir = env::current_dir().unwrap();
        let setting_path = current_dir.join("Settings.yaml");
        let s = Config::builder()
            .add_source(File::from(setting_path))
            .add_source(Environment::with_prefix("").prefix_separator("").separator("_"))
            .build()?;
        s.try_deserialize()
    }
}

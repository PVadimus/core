use typeshare::typeshare;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Equatable, Codable, CaseIterable")]
pub enum Currency {
    MXN,
    CHF,
    CNY,
    THB,
    HUF,
    AUD,
    IDR,
    RUB,
    ZAR,
    EUR,
    NZD,
    SAR,
    SGD,
    BMD,
    KWD,
    HKD,
    JPY,
    GBP,
    DKK,
    KRW,
    PHP,
    CLP,
    TWD,
    PKR,
    BRL,
    CAD,
    BHD,
    MMK,
    VEF,
    VND,
    CZK,
    TRY,
    INR,
    ARS,
    BDT,
    NOK,
    USD,
    LKR,
    ILS,
    PLN,
    NGN,
    UAH,
    XDR,
    MYR,
    AED,
    SEK,
}
 
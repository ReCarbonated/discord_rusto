use serde::Deserialize;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
            application_command::{CommandDataOption, CommandDataOptionValue},
            command::CommandOptionType,
        },
    prelude::Context,
};
use strum_macros::{EnumString, Display};
use std::str::FromStr;

use crate::WebClient;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("exchange")
        .description("exchange root for the bot")
        .create_option(|opt| {
            opt.name("rate")
                .description("Toggle the specified listener")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|subopt| {
                    subopt
                        .name("from")
                        .description("Currency to convert from")
                        .required(true)
                        .kind(CommandOptionType::String)
                })
                .create_sub_option(|subopt| {
                    subopt
                        .name("to")
                        .description("Currency to convert to")
                        .required(true)
                        .kind(CommandOptionType::String)
                })
        })
}

pub async fn run(options: &[CommandDataOption], ctx: &Context) -> String {
    if let Some(command_data_option) = options.get(0) {
        match command_data_option.name.as_str() {
            "rate" => {
                let from_value = command_data_option
                    .options
                    .get(0)
                    .unwrap()
                    .resolved
                    .as_ref()
                    .unwrap();
                let to_value = command_data_option
                    .options
                    .get(1)
                    .unwrap()
                    .resolved
                    .as_ref()
                    .unwrap();
                if let (
                    CommandDataOptionValue::String(from_val),
                    CommandDataOptionValue::String(to_val),
                ) = (from_value, to_value)
                {
                    // Try to make enums out of the thingies, else error out
                    if let Ok(from_currency) = Currency::from_str(&from_val.to_uppercase()) {
                        if let Ok(to_currency) = Currency::from_str(&to_val.to_uppercase()){
                            get_rate(&from_currency, &to_currency, ctx).await
                        } else {
                            format!("Failed to parse to currency: {}", to_val)
                        }
                    } else {
                        format!("Failed to parse from currency: {}", from_val)
                    }
                } else {
                    "Something blew up".to_string()
                }
            }
            _ => "Didn't get nothin".to_string(),
        }
    } else {
        "Something fucking broke".to_string()
    }
}

async fn get_rate(from: &Currency, to: &Currency, ctx: &Context) -> String {
    // Get the web client
    let client = {
        let data = ctx.data.read().await;
        data.get::<WebClient>()
        .expect("Expected WebClient in TypeMap")
        .clone()
    };

    let output = client.get(
        format!("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{}/{}.min.json", from.to_string().to_lowercase(), to.to_string().to_lowercase())
    )
    .send()
    .await
    .unwrap()
    .json::<ExchangeRate>()
    .await
    .unwrap();

    match output.rate {
        CurrencyRate::AED(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::AFN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ALL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::AMD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ANG(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::AOA(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ARS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::AUD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::AWG(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::AZN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BAM(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BBD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BDT(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BGN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BHD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BIF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BMD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BND(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BOB(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BRL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BSD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BTC(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BTN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BWP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BYN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::BZD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CAD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CDF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CHF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CLF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CLP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CNH(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CNY(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::COP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CRC(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CUC(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CUP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CVE(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::CZK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::DJF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::DKK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::DOP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::DZD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::EGP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ERN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ETB(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::EUR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::FJD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::FKP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GBP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GEL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GGP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GHS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GIP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GMD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GNF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GTQ(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::GYD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::HKD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::HNL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::HRK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::HTG(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::HUF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::IDR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ILS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::IMP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::INR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::IQD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::IRR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ISK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::JEP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::JMD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::JOD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::JPY(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KES(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KGS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KHR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KMF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KPW(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KRW(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KWD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KYD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::KZT(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::LAK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::LBP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::LKR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::LRD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::LSL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::LYD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MAD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MDL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MGA(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MKD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MMK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MNT(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MOP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MRO(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MRU(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MUR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MVR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MWK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MXN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MYR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::MZN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::NAD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::NGN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::NIO(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::NOK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::NPR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::NZD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::OMR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::PAB(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::PEN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::PGK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::PHP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::PKR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::PLN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::PYG(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::QAR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::RON(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::RSD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::RUB(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::RWF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SAR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SBD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SCR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SDG(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SEK(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SGD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SHP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SLL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SOS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SRD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SSP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::STD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::STN(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SVC(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SYP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::SZL(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::THB(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TJS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TMT(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TND(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TOP(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TRY(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TTD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TWD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::TZS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::UAH(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::UGX(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::USD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::UYU(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::UZS(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::VEF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::VND(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::VUV(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::WST(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XAF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XAG(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XAU(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XCD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XDR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XOF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XPD(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XPF(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::XPT(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::YER(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ZAR(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ZMW(output) => {format!("1{} to {} {}", from, output, to)},
        CurrencyRate::ZWL(output) => {format!("1{} to {} {}", from, output, to)},
    }
}

#[derive(Deserialize, Debug)]
pub struct ExchangeRate {
    pub date: String,
    #[serde(flatten)]
    pub rate: CurrencyRate,
}


#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Currency {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BTC,
    BTN,
    BWP,
    BYN,
    BZD,
    CAD,
    CDF,
    CHF,
    CLF,
    CLP,
    CNH,
    CNY,
    COP,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GGP,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    IMP,
    INR,
    IQD,
    IRR,
    ISK,
    JEP,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRO,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    SSP,
    STD,
    STN,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    UYU,
    UZS,
    VEF,
    VND,
    VUV,
    WST,
    XAF,
    XAG,
    XAU,
    XCD,
    XDR,
    XOF,
    XPD,
    XPF,
    XPT,
    YER,
    ZAR,
    ZMW,
    ZWL,
}

#[derive(Deserialize, Debug)]
pub enum CurrencyRate {
    #[serde(rename = "aed")]
    AED(f32),
    #[serde(rename = "afn")]
    AFN(f32),
    #[serde(rename = "all")]
    ALL(f32),
    #[serde(rename = "amd")]
    AMD(f32),
    #[serde(rename = "ang")]
    ANG(f32),
    #[serde(rename = "aoa")]
    AOA(f32),
    #[serde(rename = "ars")]
    ARS(f32),
    #[serde(rename = "aud")]
    AUD(f32),
    #[serde(rename = "awg")]
    AWG(f32),
    #[serde(rename = "azn")]
    AZN(f32),
    #[serde(rename = "bam")]
    BAM(f32),
    #[serde(rename = "bbd")]
    BBD(f32),
    #[serde(rename = "bdt")]
    BDT(f32),
    #[serde(rename = "bgn")]
    BGN(f32),
    #[serde(rename = "bhd")]
    BHD(f32),
    #[serde(rename = "bif")]
    BIF(f32),
    #[serde(rename = "bmd")]
    BMD(f32),
    #[serde(rename = "bnd")]
    BND(f32),
    #[serde(rename = "bob")]
    BOB(f32),
    #[serde(rename = "brl")]
    BRL(f32),
    #[serde(rename = "bsd")]
    BSD(f32),
    #[serde(rename = "btc")]
    BTC(f32),
    #[serde(rename = "btn")]
    BTN(f32),
    #[serde(rename = "bwp")]
    BWP(f32),
    #[serde(rename = "byn")]
    BYN(f32),
    #[serde(rename = "bzd")]
    BZD(f32),
    #[serde(rename = "cad")]
    CAD(f32),
    #[serde(rename = "cdf")]
    CDF(f32),
    #[serde(rename = "chf")]
    CHF(f32),
    #[serde(rename = "clf")]
    CLF(f32),
    #[serde(rename = "clp")]
    CLP(f32),
    #[serde(rename = "cnh")]
    CNH(f32),
    #[serde(rename = "cny")]
    CNY(f32),
    #[serde(rename = "cop")]
    COP(f32),
    #[serde(rename = "crc")]
    CRC(f32),
    #[serde(rename = "cuc")]
    CUC(f32),
    #[serde(rename = "cup")]
    CUP(f32),
    #[serde(rename = "cve")]
    CVE(f32),
    #[serde(rename = "czk")]
    CZK(f32),
    #[serde(rename = "djk")]
    DJF(f32),
    #[serde(rename = "dkk")]
    DKK(f32),
    #[serde(rename = "dop")]
    DOP(f32),
    #[serde(rename = "dzd")]
    DZD(f32),
    #[serde(rename = "egp")]
    EGP(f32),
    #[serde(rename = "ern")]
    ERN(f32),
    #[serde(rename = "etb")]
    ETB(f32),
    #[serde(rename = "eur")]
    EUR(f32),
    #[serde(rename = "fjd")]
    FJD(f32),
    #[serde(rename = "fkp")]
    FKP(f32),
    #[serde(rename = "gbp")]
    GBP(f32),
    #[serde(rename = "gel")]
    GEL(f32),
    #[serde(rename = "ggp")]
    GGP(f32),
    #[serde(rename = "ghs")]
    GHS(f32),
    #[serde(rename = "gip")]
    GIP(f32),
    #[serde(rename = "gmd")]
    GMD(f32),
    #[serde(rename = "gnf")]
    GNF(f32),
    #[serde(rename = "gtq")]
    GTQ(f32),
    #[serde(rename = "gyd")]
    GYD(f32),
    #[serde(rename = "hkd")]
    HKD(f32),
    #[serde(rename = "hnl")]
    HNL(f32),
    #[serde(rename = "hrk")]
    HRK(f32),
    #[serde(rename = "htg")]
    HTG(f32),
    #[serde(rename = "huf")]
    HUF(f32),
    #[serde(rename = "idr")]
    IDR(f32),
    #[serde(rename = "ils")]
    ILS(f32),
    #[serde(rename = "imp")]
    IMP(f32),
    #[serde(rename = "inr")]
    INR(f32),
    #[serde(rename = "iqd")]
    IQD(f32),
    #[serde(rename = "irr")]
    IRR(f32),
    #[serde(rename = "isk")]
    ISK(f32),
    #[serde(rename = "jep")]
    JEP(f32),
    #[serde(rename = "jmd")]
    JMD(f32),
    #[serde(rename = "jod")]
    JOD(f32),
    #[serde(rename = "jpy")]
    JPY(f32),
    #[serde(rename = "kes")]
    KES(f32),
    #[serde(rename = "kgs")]
    KGS(f32),
    #[serde(rename = "khr")]
    KHR(f32),
    #[serde(rename = "kmf")]
    KMF(f32),
    #[serde(rename = "kpw")]
    KPW(f32),
    #[serde(rename = "krw")]
    KRW(f32),
    #[serde(rename = "kwd")]
    KWD(f32),
    #[serde(rename = "kyd")]
    KYD(f32),
    #[serde(rename = "kzt")]
    KZT(f32),
    #[serde(rename = "lak")]
    LAK(f32),
    #[serde(rename = "lbp")]
    LBP(f32),
    #[serde(rename = "lkr")]
    LKR(f32),
    #[serde(rename = "lrd")]
    LRD(f32),
    #[serde(rename = "lsl")]
    LSL(f32),
    #[serde(rename = "lyd")]
    LYD(f32),
    #[serde(rename = "mad")]
    MAD(f32),
    #[serde(rename = "mdl")]
    MDL(f32),
    #[serde(rename = "mga")]
    MGA(f32),
    #[serde(rename = "mkd")]
    MKD(f32),
    #[serde(rename = "mmk")]
    MMK(f32),
    #[serde(rename = "mnt")]
    MNT(f32),
    #[serde(rename = "mop")]
    MOP(f32),
    #[serde(rename = "mro")]
    MRO(f32),
    #[serde(rename = "mru")]
    MRU(f32),
    #[serde(rename = "mur")]
    MUR(f32),
    #[serde(rename = "mvr")]
    MVR(f32),
    #[serde(rename = "mwk")]
    MWK(f32),
    #[serde(rename = "mxn")]
    MXN(f32),
    #[serde(rename = "myr")]
    MYR(f32),
    #[serde(rename = "mzn")]
    MZN(f32),
    #[serde(rename = "nad")]
    NAD(f32),
    #[serde(rename = "ngn")]
    NGN(f32),
    #[serde(rename = "nio")]
    NIO(f32),
    #[serde(rename = "nok")]
    NOK(f32),
    #[serde(rename = "npr")]
    NPR(f32),
    #[serde(rename = "nzd")]
    NZD(f32),
    #[serde(rename = "omr")]
    OMR(f32),
    #[serde(rename = "pab")]
    PAB(f32),
    #[serde(rename = "pen")]
    PEN(f32),
    #[serde(rename = "pgk")]
    PGK(f32),
    #[serde(rename = "php")]
    PHP(f32),
    #[serde(rename = "pkr")]
    PKR(f32),
    #[serde(rename = "pln")]
    PLN(f32),
    #[serde(rename = "pyg")]
    PYG(f32),
    #[serde(rename = "qar")]
    QAR(f32),
    #[serde(rename = "ron")]
    RON(f32),
    #[serde(rename = "rsd")]
    RSD(f32),
    #[serde(rename = "rub")]
    RUB(f32),
    #[serde(rename = "rwf")]
    RWF(f32),
    #[serde(rename = "sar")]
    SAR(f32),
    #[serde(rename = "sbd")]
    SBD(f32),
    #[serde(rename = "scr")]
    SCR(f32),
    #[serde(rename = "sdg")]
    SDG(f32),
    #[serde(rename = "sek")]
    SEK(f32),
    #[serde(rename = "sgd")]
    SGD(f32),
    #[serde(rename = "shp")]
    SHP(f32),
    #[serde(rename = "sll")]
    SLL(f32),
    #[serde(rename = "sos")]
    SOS(f32),
    #[serde(rename = "srd")]
    SRD(f32),
    #[serde(rename = "ssp")]
    SSP(f32),
    #[serde(rename = "std")]
    STD(f32),
    #[serde(rename = "stn")]
    STN(f32),
    #[serde(rename = "svc")]
    SVC(f32),
    #[serde(rename = "syp")]
    SYP(f32),
    #[serde(rename = "szl")]
    SZL(f32),
    #[serde(rename = "thb")]
    THB(f32),
    #[serde(rename = "tjs")]
    TJS(f32),
    #[serde(rename = "tmt")]
    TMT(f32),
    #[serde(rename = "tnd")]
    TND(f32),
    #[serde(rename = "top")]
    TOP(f32),
    #[serde(rename = "try")]
    TRY(f32),
    #[serde(rename = "ttd")]
    TTD(f32),
    #[serde(rename = "twd")]
    TWD(f32),
    #[serde(rename = "tzs")]
    TZS(f32),
    #[serde(rename = "uah")]
    UAH(f32),
    #[serde(rename = "ugx")]
    UGX(f32),
    #[serde(rename = "usd")]
    USD(f32),
    #[serde(rename = "uyu")]
    UYU(f32),
    #[serde(rename = "uzs")]
    UZS(f32),
    #[serde(rename = "vef")]
    VEF(f32),
    #[serde(rename = "vnd")]
    VND(f32),
    #[serde(rename = "vuv")]
    VUV(f32),
    #[serde(rename = "wst")]
    WST(f32),
    #[serde(rename = "xaf")]
    XAF(f32),
    #[serde(rename = "xag")]
    XAG(f32),
    #[serde(rename = "xau")]
    XAU(f32),
    #[serde(rename = "xcd")]
    XCD(f32),
    #[serde(rename = "xdr")]
    XDR(f32),
    #[serde(rename = "xof")]
    XOF(f32),
    #[serde(rename = "xpd")]
    XPD(f32),
    #[serde(rename = "xpf")]
    XPF(f32),
    #[serde(rename = "xpt")]
    XPT(f32),
    #[serde(rename = "yer")]
    YER(f32),
    #[serde(rename = "zar")]
    ZAR(f32),
    #[serde(rename = "zmw")]
    ZMW(f32),
    #[serde(rename = "zwl")]
    ZWL(f32),
}

use serde::Deserialize;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::{CommandDataOption, CommandDataOptionValue},
        command::CommandOptionType,
    },
    prelude::Context,
};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

use crate::WebClient;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("exchange")
        .description("exchange root for the bot")
        .create_option(|opt| {
            opt.name("rate")
                .description("Gets daily conversion rate, uses ISO-4217 codes")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|subopt| {
                    subopt
                        .name("from")
                        .description("Currency to convert from, (USD, CAD, JPY, EUR)")
                        .required(true)
                        .kind(CommandOptionType::String)
                })
                .create_sub_option(|subopt| {
                    subopt
                        .name("to")
                        .description("Currency to convert to (VND, MYR, ZWL")
                        .required(true)
                        .kind(CommandOptionType::String)
                })
                .create_sub_option(|sub_opt| {
                    sub_opt
                        .name("amount")
                        .description("Amount to convert from base currency")
                        .kind(CommandOptionType::Number)
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
                let amount;
                if let Some(possible_number) = command_data_option.options.get(2) {
                    if let Some(resolved_number) = possible_number.resolved.as_ref() {
                        if let CommandDataOptionValue::Number(parsed_amount) = resolved_number {
                            amount = parsed_amount.clone();
                        } else {
                            amount = 1.0;
                        }
                    } else {
                        amount = 1.0;
                    }
                } else {
                    amount = 1.0;
                }
                if let (
                    CommandDataOptionValue::String(from_val),
                    CommandDataOptionValue::String(to_val),
                ) = (from_value, to_value)
                {
                    // Try to make enums out of the thingies, else error out
                    if let Ok(from_currency) = Currency::from_str(&from_val.to_uppercase()) {
                        if let Ok(to_currency) = Currency::from_str(&to_val.to_uppercase()) {
                            get_rate(&from_currency, &to_currency, ctx, amount).await
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

async fn get_rate(from: &Currency, to: &Currency, ctx: &Context, amount: f64) -> String {
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
        CurrencyRate::AED(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::AFN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ALL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::AMD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ANG(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::AOA(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ARS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::AUD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::AWG(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::AZN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BAM(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BBD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BDT(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BGN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BHD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BIF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BMD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BND(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BOB(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BRL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BSD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BTC(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BTN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BWP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BYN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::BZD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CAD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CDF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CHF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CLF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CLP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CNH(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CNY(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::COP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CRC(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CUC(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CUP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CVE(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::CZK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::DJF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::DKK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::DOP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::DZD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::EGP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ERN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ETB(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::EUR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::FJD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::FKP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GBP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GEL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GGP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GHS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GIP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GMD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GNF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GTQ(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::GYD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::HKD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::HNL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::HRK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::HTG(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::HUF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::IDR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ILS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::IMP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::INR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::IQD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::IRR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ISK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::JEP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::JMD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::JOD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::JPY(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KES(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KGS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KHR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KMF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KPW(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KRW(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KWD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KYD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::KZT(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::LAK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::LBP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::LKR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::LRD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::LSL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::LYD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MAD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MDL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MGA(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MKD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MMK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MNT(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MOP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MRO(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MRU(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MUR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MVR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MWK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MXN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MYR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::MZN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::NAD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::NGN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::NIO(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::NOK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::NPR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::NZD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::OMR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::PAB(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::PEN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::PGK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::PHP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::PKR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::PLN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::PYG(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::QAR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::RON(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::RSD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::RUB(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::RWF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SAR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SBD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SCR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SDG(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SEK(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SGD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SHP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SLL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SOS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SRD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SSP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::STD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::STN(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SVC(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SYP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::SZL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::THB(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TJS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TMT(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TND(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TOP(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TRY(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TTD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TWD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::TZS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::UAH(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::UGX(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::USD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::UYU(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::UZS(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::VEF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::VND(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::VUV(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::WST(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XAF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XAG(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XAU(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XCD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XDR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XOF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XPD(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XPF(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::XPT(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::YER(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ZAR(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ZMW(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
        CurrencyRate::ZWL(output) => {
            format!("{} {} to {} {}",amount, from, output*amount, to)
        }
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
    AED(f64),
    #[serde(rename = "afn")]
    AFN(f64),
    #[serde(rename = "all")]
    ALL(f64),
    #[serde(rename = "amd")]
    AMD(f64),
    #[serde(rename = "ang")]
    ANG(f64),
    #[serde(rename = "aoa")]
    AOA(f64),
    #[serde(rename = "ars")]
    ARS(f64),
    #[serde(rename = "aud")]
    AUD(f64),
    #[serde(rename = "awg")]
    AWG(f64),
    #[serde(rename = "azn")]
    AZN(f64),
    #[serde(rename = "bam")]
    BAM(f64),
    #[serde(rename = "bbd")]
    BBD(f64),
    #[serde(rename = "bdt")]
    BDT(f64),
    #[serde(rename = "bgn")]
    BGN(f64),
    #[serde(rename = "bhd")]
    BHD(f64),
    #[serde(rename = "bif")]
    BIF(f64),
    #[serde(rename = "bmd")]
    BMD(f64),
    #[serde(rename = "bnd")]
    BND(f64),
    #[serde(rename = "bob")]
    BOB(f64),
    #[serde(rename = "brl")]
    BRL(f64),
    #[serde(rename = "bsd")]
    BSD(f64),
    #[serde(rename = "btc")]
    BTC(f64),
    #[serde(rename = "btn")]
    BTN(f64),
    #[serde(rename = "bwp")]
    BWP(f64),
    #[serde(rename = "byn")]
    BYN(f64),
    #[serde(rename = "bzd")]
    BZD(f64),
    #[serde(rename = "cad")]
    CAD(f64),
    #[serde(rename = "cdf")]
    CDF(f64),
    #[serde(rename = "chf")]
    CHF(f64),
    #[serde(rename = "clf")]
    CLF(f64),
    #[serde(rename = "clp")]
    CLP(f64),
    #[serde(rename = "cnh")]
    CNH(f64),
    #[serde(rename = "cny")]
    CNY(f64),
    #[serde(rename = "cop")]
    COP(f64),
    #[serde(rename = "crc")]
    CRC(f64),
    #[serde(rename = "cuc")]
    CUC(f64),
    #[serde(rename = "cup")]
    CUP(f64),
    #[serde(rename = "cve")]
    CVE(f64),
    #[serde(rename = "czk")]
    CZK(f64),
    #[serde(rename = "djk")]
    DJF(f64),
    #[serde(rename = "dkk")]
    DKK(f64),
    #[serde(rename = "dop")]
    DOP(f64),
    #[serde(rename = "dzd")]
    DZD(f64),
    #[serde(rename = "egp")]
    EGP(f64),
    #[serde(rename = "ern")]
    ERN(f64),
    #[serde(rename = "etb")]
    ETB(f64),
    #[serde(rename = "eur")]
    EUR(f64),
    #[serde(rename = "fjd")]
    FJD(f64),
    #[serde(rename = "fkp")]
    FKP(f64),
    #[serde(rename = "gbp")]
    GBP(f64),
    #[serde(rename = "gel")]
    GEL(f64),
    #[serde(rename = "ggp")]
    GGP(f64),
    #[serde(rename = "ghs")]
    GHS(f64),
    #[serde(rename = "gip")]
    GIP(f64),
    #[serde(rename = "gmd")]
    GMD(f64),
    #[serde(rename = "gnf")]
    GNF(f64),
    #[serde(rename = "gtq")]
    GTQ(f64),
    #[serde(rename = "gyd")]
    GYD(f64),
    #[serde(rename = "hkd")]
    HKD(f64),
    #[serde(rename = "hnl")]
    HNL(f64),
    #[serde(rename = "hrk")]
    HRK(f64),
    #[serde(rename = "htg")]
    HTG(f64),
    #[serde(rename = "huf")]
    HUF(f64),
    #[serde(rename = "idr")]
    IDR(f64),
    #[serde(rename = "ils")]
    ILS(f64),
    #[serde(rename = "imp")]
    IMP(f64),
    #[serde(rename = "inr")]
    INR(f64),
    #[serde(rename = "iqd")]
    IQD(f64),
    #[serde(rename = "irr")]
    IRR(f64),
    #[serde(rename = "isk")]
    ISK(f64),
    #[serde(rename = "jep")]
    JEP(f64),
    #[serde(rename = "jmd")]
    JMD(f64),
    #[serde(rename = "jod")]
    JOD(f64),
    #[serde(rename = "jpy")]
    JPY(f64),
    #[serde(rename = "kes")]
    KES(f64),
    #[serde(rename = "kgs")]
    KGS(f64),
    #[serde(rename = "khr")]
    KHR(f64),
    #[serde(rename = "kmf")]
    KMF(f64),
    #[serde(rename = "kpw")]
    KPW(f64),
    #[serde(rename = "krw")]
    KRW(f64),
    #[serde(rename = "kwd")]
    KWD(f64),
    #[serde(rename = "kyd")]
    KYD(f64),
    #[serde(rename = "kzt")]
    KZT(f64),
    #[serde(rename = "lak")]
    LAK(f64),
    #[serde(rename = "lbp")]
    LBP(f64),
    #[serde(rename = "lkr")]
    LKR(f64),
    #[serde(rename = "lrd")]
    LRD(f64),
    #[serde(rename = "lsl")]
    LSL(f64),
    #[serde(rename = "lyd")]
    LYD(f64),
    #[serde(rename = "mad")]
    MAD(f64),
    #[serde(rename = "mdl")]
    MDL(f64),
    #[serde(rename = "mga")]
    MGA(f64),
    #[serde(rename = "mkd")]
    MKD(f64),
    #[serde(rename = "mmk")]
    MMK(f64),
    #[serde(rename = "mnt")]
    MNT(f64),
    #[serde(rename = "mop")]
    MOP(f64),
    #[serde(rename = "mro")]
    MRO(f64),
    #[serde(rename = "mru")]
    MRU(f64),
    #[serde(rename = "mur")]
    MUR(f64),
    #[serde(rename = "mvr")]
    MVR(f64),
    #[serde(rename = "mwk")]
    MWK(f64),
    #[serde(rename = "mxn")]
    MXN(f64),
    #[serde(rename = "myr")]
    MYR(f64),
    #[serde(rename = "mzn")]
    MZN(f64),
    #[serde(rename = "nad")]
    NAD(f64),
    #[serde(rename = "ngn")]
    NGN(f64),
    #[serde(rename = "nio")]
    NIO(f64),
    #[serde(rename = "nok")]
    NOK(f64),
    #[serde(rename = "npr")]
    NPR(f64),
    #[serde(rename = "nzd")]
    NZD(f64),
    #[serde(rename = "omr")]
    OMR(f64),
    #[serde(rename = "pab")]
    PAB(f64),
    #[serde(rename = "pen")]
    PEN(f64),
    #[serde(rename = "pgk")]
    PGK(f64),
    #[serde(rename = "php")]
    PHP(f64),
    #[serde(rename = "pkr")]
    PKR(f64),
    #[serde(rename = "pln")]
    PLN(f64),
    #[serde(rename = "pyg")]
    PYG(f64),
    #[serde(rename = "qar")]
    QAR(f64),
    #[serde(rename = "ron")]
    RON(f64),
    #[serde(rename = "rsd")]
    RSD(f64),
    #[serde(rename = "rub")]
    RUB(f64),
    #[serde(rename = "rwf")]
    RWF(f64),
    #[serde(rename = "sar")]
    SAR(f64),
    #[serde(rename = "sbd")]
    SBD(f64),
    #[serde(rename = "scr")]
    SCR(f64),
    #[serde(rename = "sdg")]
    SDG(f64),
    #[serde(rename = "sek")]
    SEK(f64),
    #[serde(rename = "sgd")]
    SGD(f64),
    #[serde(rename = "shp")]
    SHP(f64),
    #[serde(rename = "sll")]
    SLL(f64),
    #[serde(rename = "sos")]
    SOS(f64),
    #[serde(rename = "srd")]
    SRD(f64),
    #[serde(rename = "ssp")]
    SSP(f64),
    #[serde(rename = "std")]
    STD(f64),
    #[serde(rename = "stn")]
    STN(f64),
    #[serde(rename = "svc")]
    SVC(f64),
    #[serde(rename = "syp")]
    SYP(f64),
    #[serde(rename = "szl")]
    SZL(f64),
    #[serde(rename = "thb")]
    THB(f64),
    #[serde(rename = "tjs")]
    TJS(f64),
    #[serde(rename = "tmt")]
    TMT(f64),
    #[serde(rename = "tnd")]
    TND(f64),
    #[serde(rename = "top")]
    TOP(f64),
    #[serde(rename = "try")]
    TRY(f64),
    #[serde(rename = "ttd")]
    TTD(f64),
    #[serde(rename = "twd")]
    TWD(f64),
    #[serde(rename = "tzs")]
    TZS(f64),
    #[serde(rename = "uah")]
    UAH(f64),
    #[serde(rename = "ugx")]
    UGX(f64),
    #[serde(rename = "usd")]
    USD(f64),
    #[serde(rename = "uyu")]
    UYU(f64),
    #[serde(rename = "uzs")]
    UZS(f64),
    #[serde(rename = "vef")]
    VEF(f64),
    #[serde(rename = "vnd")]
    VND(f64),
    #[serde(rename = "vuv")]
    VUV(f64),
    #[serde(rename = "wst")]
    WST(f64),
    #[serde(rename = "xaf")]
    XAF(f64),
    #[serde(rename = "xag")]
    XAG(f64),
    #[serde(rename = "xau")]
    XAU(f64),
    #[serde(rename = "xcd")]
    XCD(f64),
    #[serde(rename = "xdr")]
    XDR(f64),
    #[serde(rename = "xof")]
    XOF(f64),
    #[serde(rename = "xpd")]
    XPD(f64),
    #[serde(rename = "xpf")]
    XPF(f64),
    #[serde(rename = "xpt")]
    XPT(f64),
    #[serde(rename = "yer")]
    YER(f64),
    #[serde(rename = "zar")]
    ZAR(f64),
    #[serde(rename = "zmw")]
    ZMW(f64),
    #[serde(rename = "zwl")]
    ZWL(f64),
}

use std::fs::File;
// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;

use csv;
use std::net::IpAddr;
use maxminddb::geoip2;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "MCC")]
    mcc: String,
    #[serde(rename = "MCC (int)")]
    mcc_int: u16,
    #[serde(rename = "MNC")]
    mnc: String,
    #[serde(rename = "MNC (int)")]
    mnc_int: u16,
    #[serde(rename = "ISO")]
    iso: String,
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "Country Code")]
    country_code: Option<u16>,
    #[serde(rename = "Network")]
    network: String,
}

pub fn get_client_ip_from_x_forwarded_for(x_forwarded_for: &str) -> Option<String> {
    x_forwarded_for
        .split(",")
        .next()
        .map(String::from)
}

pub fn resolve_ip_to_country_code(ip: &str) -> Option<String> {
    let reader = maxminddb::Reader::open_readfile("GeoLite2-Country.mmdb").unwrap();
    let ip_addr: IpAddr = ip.parse().unwrap();

    let country: geoip2::Country = reader.lookup(ip_addr).unwrap();
    let country_iso_code = country.country.unwrap().iso_code;
    
    country_iso_code.map(String::from)
}

fn resolve_mcc_to_country_code(mcc: &str) -> Option<String> {
    let mut rdr = csv::Reader::from_reader(File::open("mcc-mnc-table.csv").unwrap());
    let mcc_to_country_iso: HashMap<String, String> = rdr.deserialize().map(|result| {
        let record: Record = result.unwrap();
        (record.mcc, record.iso)
    })
    .collect();

    mcc_to_country_iso.get(mcc).map(String::from)
}

fn main() {
    println!("Start");
    let ip_from_xff = get_client_ip_from_x_forwarded_for("121.101.11.55, 10.1.10.1");
    dbg!(&ip_from_xff);
    let ip_to_country_code = resolve_ip_to_country_code(&ip_from_xff.unwrap());
    dbg!(&ip_to_country_code);
    let mcc_to_country_code = resolve_mcc_to_country_code("450");
    dbg!(&mcc_to_country_code);
}

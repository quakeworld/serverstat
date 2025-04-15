use anyhow::Error;
use phf::phf_map;
use quake_serverinfo::Settings;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct GeoInfo {
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub coords: Option<Coordinates>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

impl Eq for Coordinates {
    fn assert_receiver_is_total_eq(&self) {
        // This is a no-op, but it allows us to implement `Eq` for `Coordinates`.
        // The default implementation of `PartialEq` is sufficient for our needs.
    }
}

impl TryFrom<&str> for Coordinates {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split_once(',');

        if let Some((lat_str, lng_str)) = parts {
            let lat = lat_str.trim().parse::<f64>()?;
            let long = lng_str.trim().parse::<f64>()?;
            Ok(Self { lat, lng: long })
        } else {
            Err(Error::msg("Invalid coordinate format"))
        }
    }
}

impl From<&Settings> for GeoInfo {
    fn from(settings: &Settings) -> Self {
        let country_code = settings
            .countrycode
            .clone()
            .map(|cc| cc.to_uppercase().trim().to_string());

        let (country_name, region) = country_code
            .clone()
            .map(|cc| info_by_cc(&cc))
            .unwrap_or_default();

        let coords = match &settings.coords {
            Some(coords) => Coordinates::try_from(coords.as_str()).ok(),
            None => None,
        };

        Self {
            country_code,
            country_name,
            city: settings.city.clone(),
            region,
            coords,
        }
    }
}

fn info_by_cc(code: &str) -> (Option<String>, Option<String>) {
    COUNTRY_INFO
        .get(code)
        .map(|(name, region)| (Some(name.to_string()), Some(region.to_string())))
        .unwrap_or_default()
}

#[allow(dead_code)]
static COUNTRY_INFO: phf::Map<&'static str, (&'static str, &'static str)> = phf_map! {
    "AD" => ("Andorra", "Europe"),
    "AE" => ("United Arab Emirates", "Asia"),
    "AF" => ("Afghanistan", "Asia"),
    "AG" => ("Antigua and Barbuda", "North America"),
    "AI" => ("Anguilla", "North America"),
    "AL" => ("Albania", "Europe"),
    "AM" => ("Armenia", "Asia"),
    "AO" => ("Angola", "Africa"),
    "AQ" => ("Antarctica", "Antarctica"),
    "AR" => ("Argentina", "South America"),
    "AS" => ("American Samoa", "North America"),
    "AT" => ("Austria", "Europe"),
    "AU" => ("Australia", "Oceania"),
    "AW" => ("Aruba", "North America"),
    "AX" => ("Åland Islands", "Europe"),
    "AZ" => ("Azerbaijan", "Asia"),
    "BA" => ("Bosnia and Herzegovina", "Europe"),
    "BB" => ("Barbados", "North America"),
    "BD" => ("Bangladesh", "Asia"),
    "BE" => ("Belgium", "Europe"),
    "BF" => ("Burkina Faso", "Africa"),
    "BG" => ("Bulgaria", "Europe"),
    "BH" => ("Bahrain", "Asia"),
    "BI" => ("Burundi", "Africa"),
    "BJ" => ("Benin", "Africa"),
    "BL" => ("Saint Barthélemy", "North America"),
    "BM" => ("Bermuda", "North America"),
    "BN" => ("Brunei Darussalam", "Asia"),
    "BO" => ("Bolivia", "South America"),
    "BQ" => ("Bonaire", "North America"),
    "BR" => ("Brazil", "South America"),
    "BS" => ("Bahamas", "North America"),
    "BT" => ("Bhutan", "Asia"),
    "BV" => ("Bouvet Island", "Antarctica"),
    "BW" => ("Botswana", "Africa"),
    "BY" => ("Belarus", "Europe"),
    "BZ" => ("Belize", "North America"),
    "CA" => ("Canada", "North America"),
    "CC" => ("Cocos (Keeling) Islands", "Asia"),
    "CD" => ("Congo", "Africa"),
    "CF" => ("Central African Republic", "Africa"),
    "CG" => ("Congo", "Africa"),
    "CH" => ("Switzerland", "Europe"),
    "CI" => ("Côte d'Ivoire", "Africa"),
    "CK" => ("Cook Islands", "Oceania"),
    "CL" => ("Chile", "South America"),
    "CM" => ("Cameroon", "Africa"),
    "CN" => ("China", "Asia"),
    "CO" => ("Colombia", "South America"),
    "CR" => ("Costa Rica", "North America"),
    "CU" => ("Cuba", "North America"),
    "CV" => ("Cape Verde", "Africa"),
    "CW" => ("Curaçao", "North America"),
    "CX" => ("Christmas Island", "Oceania"),
    "CY" => ("Cyprus", "Europe"),
    "CZ" => ("Czech Republic", "Europe"),
    "DE" => ("Germany", "Europe"),
    "DJ" => ("Djibouti", "Africa"),
    "DK" => ("Denmark", "Europe"),
    "DM" => ("Dominica", "North America"),
    "DO" => ("Dominican Republic", "North America"),
    "DZ" => ("Algeria", "Africa"),
    "EC" => ("Ecuador", "South America"),
    "EE" => ("Estonia", "Europe"),
    "EG" => ("Egypt", "Africa"),
    "EH" => ("Western Sahara", "Africa"),
    "ER" => ("Eritrea", "Africa"),
    "ES" => ("Spain", "Europe"),
    "ET" => ("Ethiopia", "Africa"),
    "FI" => ("Finland", "Europe"),
    "FJ" => ("Fiji", "Oceania"),
    "FK" => ("Falkland Islands (Malvinas)", "South America"),
    "FM" => ("Micronesia", "Oceania"),
    "FO" => ("Faroe Islands", "Europe"),
    "FR" => ("France", "Europe"),
    "GA" => ("Gabon", "Africa"),
    "GB" => ("United Kingdom", "Europe"),
    "GD" => ("Grenada", "North America"),
    "GE" => ("Georgia", "Asia"),
    "GF" => ("French Guiana", "South America"),
    "GG" => ("Guernsey", "Europe"),
    "GH" => ("Ghana", "Africa"),
    "GI" => ("Gibraltar", "Europe"),
    "GL" => ("Greenland", "North America"),
    "GM" => ("Gambia", "Africa"),
    "GN" => ("Guinea", "Africa"),
    "GP" => ("Guadeloupe", "North America"),
    "GQ" => ("Equatorial Guinea", "Africa"),
    "GR" => ("Greece", "Europe"),
    "GS" => ("South Georgia and the South Sandwich Islands", "South America"),
    "GT" => ("Guatemala", "North America"),
    "GU" => ("Guam", "Oceania"),
    "GW" => ("Guinea-Bissau", "Africa"),
    "GY" => ("Guyana", "South America"),
    "HK" => ("Hong Kong", "Asia"),
    "HM" => ("Heard Island and McDonald Islands", "Oceania"),
    "HN" => ("Honduras", "North America"),
    "HR" => ("Croatia", "Europe"),
    "HT" => ("Haiti", "North America"),
    "HU" => ("Hungary", "Europe"),
    "ID" => ("Indonesia", "Asia"),
    "IE" => ("Ireland", "Europe"),
    "IL" => ("Israel", "Asia"),
    "IM" => ("Isle of Man", "Europe"),
    "IN" => ("India", "Asia"),
    "IO" => ("British Indian Ocean Territory", "Asia"),
    "IQ" => ("Iraq", "Asia"),
    "IR" => ("Iran", "Asia"),
    "IS" => ("Iceland", "Europe"),
    "IT" => ("Italy", "Europe"),
    "JE" => ("Jersey", "Europe"),
    "JM" => ("Jamaica", "North America"),
    "JO" => ("Jordan", "Asia"),
    "JP" => ("Japan", "Asia"),
    "KE" => ("Kenya", "Africa"),
    "KG" => ("Kyrgyzstan", "Asia"),
    "KH" => ("Cambodia", "Asia"),
    "KI" => ("Kiribati", "Oceania"),
    "KM" => ("Comoros", "Africa"),
    "KN" => ("Saint Kitts and Nevis", "North America"),
    "KP" => ("North Korea", "Asia"),
    "KR" => ("South Korea", "Asia"),
    "KW" => ("Kuwait", "Asia"),
    "KY" => ("Cayman Islands", "North America"),
    "KZ" => ("Kazakhstan", "Asia"),
    "LA" => ("Lao", "Asia"),
    "LB" => ("Lebanon", "Asia"),
    "LC" => ("Saint Lucia", "North America"),
    "LI" => ("Liechtenstein", "Europe"),
    "LK" => ("Sri Lanka", "Asia"),
    "LR" => ("Liberia", "Africa"),
    "LS" => ("Lesotho", "Africa"),
    "LT" => ("Lithuania", "Europe"),
    "LU" => ("Luxembourg", "Europe"),
    "LV" => ("Latvia", "Europe"),
    "LY" => ("Libya", "Africa"),
    "MA" => ("Morocco", "Africa"),
    "MC" => ("Monaco", "Europe"),
    "MD" => ("Moldova", "Europe"),
    "ME" => ("Montenegro", "Europe"),
    "MF" => ("Saint Martin", "North America"),
    "MG" => ("Madagascar", "Africa"),
    "MH" => ("Marshall Islands", "Oceania"),
    "MK" => ("Macedonia", "Europe"),
    "ML" => ("Mali", "Africa"),
    "MM" => ("Myanmar", "Asia"),
    "MN" => ("Mongolia", "Asia"),
    "MO" => ("Macao", "Asia"),
    "MP" => ("Northern Mariana Islands", "Oceania"),
    "MQ" => ("Martinique", "North America"),
    "MR" => ("Mauritania", "Africa"),
    "MS" => ("Montserrat", "North America"),
    "MT" => ("Malta", "Europe"),
    "MU" => ("Mauritius", "Africa"),
    "MV" => ("Maldives", "Asia"),
    "MW" => ("Malawi", "Africa"),
    "MX" => ("Mexico", "North America"),
    "MY" => ("Malaysia", "Asia"),
    "MZ" => ("Mozambique", "Africa"),
    "NA" => ("Namibia", "Africa"),
    "NC" => ("New Caledonia", "Oceania"),
    "NE" => ("Niger", "Africa"),
    "NF" => ("Norfolk Island", "Oceania"),
    "NG" => ("Nigeria", "Africa"),
    "NI" => ("Nicaragua", "North America"),
    "NL" => ("Netherlands", "Europe"),
    "NO" => ("Norway", "Europe"),
    "NP" => ("Nepal", "Asia"),
    "NR" => ("Nauru", "Oceania"),
    "NU" => ("Niue", "Oceania"),
    "NZ" => ("New Zealand", "Oceania"),
    "OM" => ("Oman", "Asia"),
    "PA" => ("Panama", "North America"),
    "PE" => ("Peru", "South America"),
    "PF" => ("French Polynesia", "Oceania"),
    "PG" => ("Papua New Guinea", "Oceania"),
    "PH" => ("Philippines", "Asia"),
    "PK" => ("Pakistan", "Asia"),
    "PL" => ("Poland", "Europe"),
    "PM" => ("Saint Pierre and Miquelon", "North America"),
    "PN" => ("Pitcairn", "Oceania"),
    "PR" => ("Puerto Rico", "North America"),
    "PS" => ("Palestine", "Asia"),
    "PT" => ("Portugal", "Europe"),
    "PW" => ("Palau", "Oceania"),
    "PY" => ("Paraguay", "South America"),
    "QA" => ("Qatar", "Asia"),
    "RE" => ("Réunion", "Africa"),
    "RO" => ("Romania", "Europe"),
    "RS" => ("Serbia", "Europe"),
    "RU" => ("Russia", "Europe"),
    "RW" => ("Rwanda", "Africa"),
    "SA" => ("Saudi Arabia", "Asia"),
    "SB" => ("Solomon Islands", "Oceania"),
    "SC" => ("Seychelles", "Africa"),
    "SD" => ("Sudan", "Africa"),
    "SE" => ("Sweden", "Europe"),
    "SG" => ("Singapore", "Asia"),
    "SH" => ("Saint Helena", "Africa"),
    "SI" => ("Slovenia", "Europe"),
    "SJ" => ("Svalbard and Jan Mayen", "Europe"),
    "SK" => ("Slovakia", "Europe"),
    "SL" => ("Sierra Leone", "Africa"),
    "SM" => ("San Marino", "Europe"),
    "SN" => ("Senegal", "Africa"),
    "SO" => ("Somalia", "Africa"),
    "SR" => ("Suriname", "South America"),
    "SS" => ("South Sudan", "Africa"),
    "ST" => ("Sao Tome and Principe", "Africa"),
    "SV" => ("El Salvador", "North America"),
    "SX" => ("Sint Maarten (Dutch part)", "North America"),
    "SY" => ("Syrian Arab Republic", "Asia"),
    "SZ" => ("Eswatini", "Africa"),
    "TC" => ("Turks and Caicos Islands", "North America"),
    "TD" => ("Chad", "Africa"),
    "TF" => ("French Southern Territories", "Oceania"),
    "TG" => ("Togo", "Africa"),
    "TH" => ("Thailand", "Asia"),
    "TJ" => ("Tajikistan", "Asia"),
    "TK" => ("Tokelau", "Oceania"),
    "TL" => ("Timor-Leste", "Oceania"),
    "TM" => ("Turkmenistan", "Asia"),
    "TN" => ("Tunisia", "Africa"),
    "TO" => ("Tonga", "Oceania"),
    "TR" => ("Turkey", "Asia"),
    "TT" => ("Trinidad and Tobago", "North America"),
    "TV" => ("Tuvalu", "Oceania"),
    "TW" => ("Taiwan", "Asia"),
    "TZ" => ("Tanzania", "Africa"),
    "UA" => ("Ukraine", "Europe"),
    "UG" => ("Uganda", "Africa"),
    "UM" => ("United States Minor Outlying Islands", "North America"),
    "US" => ("United States", "North America"),
    "UY" => ("Uruguay", "South America"),
    "UZ" => ("Uzbekistan", "Asia"),
    "VA" => ("Holy See (Vatican City State)", "Europe"),
    "VC" => ("Saint Vincent and the Grenadines", "North America"),
    "VE" => ("Venezuela", "South America"),
    "VG" => ("Virgin Islands, British", "North America"),
    "VI" => ("Virgin Islands, U.S.", "North America"),
    "VN" => ("Viet Nam", "Asia"),
    "VU" => ("Vanuatu", "Oceania"),
    "WF" => ("Wallis and Futuna", "Oceania"),
    "WS" => ("Samoa", "Oceania"),
    "YE" => ("Yemen", "Asia"),
    "YT" => ("Mayotte", "Africa"),
    "ZA" => ("South Africa", "Africa"),
    "ZM" => ("Zambia", "Africa"),
    "ZW" => ("Zimbabwe", "Africa")
};

#[cfg(test)]
pub mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use quake_serverinfo::Settings;

    #[test]
    fn test_coordinates() -> Result<()> {
        assert_eq!(
            Coordinates::try_from("40.7128,-74.0060")?,
            Coordinates {
                lat: 40.7128,
                lng: -74.0060,
            }
        );
        assert!(Coordinates::try_from("invalid_coords").is_err());
        Ok(())
    }

    #[test]
    fn test_geo_info() {
        let settings = Settings {
            countrycode: Some("US".to_string()),
            city: Some("New York".to_string()),
            coords: Some("40.7128,-74.0060".to_string()),
            ..Default::default()
        };

        assert_eq!(
            GeoInfo::from(&settings),
            GeoInfo {
                country_code: Some("US".to_string()),
                country_name: Some("United States".to_string()),
                city: Some("New York".to_string()),
                region: Some("North America".to_string()),
                coords: Some(Coordinates {
                    lat: 40.7128,
                    lng: -74.0060,
                }),
            }
        );
    }
}

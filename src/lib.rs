use chrono::NaiveDateTime;

pub const URL_PREFIX: &str = "http://farm";
pub const URL_COMMON: &str = ".static.flickr.com/";
pub const URL_SUFFIX: &str = ".jpg";

#[derive(Debug, Clone)]
pub struct GeoTag {
    pub time: NaiveDateTime,
    pub latitude: f64,
    pub longitude: f64,
    pub serv_num: char,
    pub url_part: String,
}

impl std::fmt::Display for GeoTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\",{},{},{}{}{}{}{}",
            self.time,
            self.latitude,
            self.longitude,
            URL_PREFIX,
            self.serv_num,
            URL_COMMON,
            self.url_part,
            URL_SUFFIX
        )
    }
}


use chrono::NaiveDateTime;

pub const URL_PREFIX: &str = "http://farm";
pub const URL_COMMON: &str = ".static.flickr.com/";
pub const URL_SUFFIX: &str = ".jpg";

#[derive(Debug, Clone)]
pub struct GeoTag {
    pub time: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub domain_num: char,
    pub url_num1: u16,
    pub url_num2: u64,
}

impl GeoTag {
    pub fn to_csv_row(&self, id: u64) -> String {
        format!(
            "{},\"{}\",{},{},{}{}{}{}/{}_{:010x}{}\n",
            id,
            NaiveDateTime::from_timestamp(self.time as i64, 0),
            self.latitude,
            self.longitude,
            URL_PREFIX,
            self.domain_num,
            URL_COMMON,
            self.url_num1,
            id,
            self.url_num2,
            URL_SUFFIX
        )
    }
}

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;

pub const URL_PREFIX: &str = "http://farm";
pub const URL_COMMON: &str = ".static.flickr.com/";
pub const URL_SUFFIX: &str = ".jpg";

#[derive(Debug, Clone)]
pub struct GeoTag {
    pub time: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub domain_num: u8,
    pub url_num1: u16,
    pub url_num2: u64,
}

impl GeoTag {
    pub fn to_csv_row(&self, id: u64) -> String {
        format!(
            "{},\"{}\",{},{},{}\n",
            id,
            NaiveDateTime::from_timestamp(self.time as i64, 0),
            self.latitude,
            self.longitude,
            self.get_url(id)
        )
    }

    pub fn get_url(&self, id: u64) -> String {
        format!(
            "{}{}{}{}/{}_{:010x}{}",
            URL_PREFIX, self.domain_num, URL_COMMON, self.url_num1, id, self.url_num2, URL_SUFFIX
        )
    }

    pub fn from_str_to_geotag(s: &str) -> Result<(u64, GeoTag)> {
        let mut s = s.split(',');
        let id = s.next().ok_or_else(|| anyhow!("Id missing"))?.parse()?;
        let time = s.next().ok_or_else(|| anyhow!("Time missing"))?.parse()?;
        let latitude = s
            .next()
            .ok_or_else(|| anyhow!("Latitude missing"))?
            .parse()?;
        let longitude = s
            .next()
            .ok_or_else(|| anyhow!("Longitude missing"))?
            .parse()?;
        let domain_num = s
            .next()
            .ok_or_else(|| anyhow!("Serv_num missing"))?
            .parse()?;
        let url_num1 = s
            .next()
            .ok_or_else(|| anyhow!("Url_num1 missing"))?
            .parse()?;
        let url_num2 =
            u64::from_str_radix(s.next().ok_or_else(|| anyhow!("Url_num2 missing"))?, 16)?;

        Ok((
            id,
            GeoTag {
                time,
                latitude,
                longitude,
                domain_num,
                url_num1,
                url_num2,
            },
        ))
    }
}

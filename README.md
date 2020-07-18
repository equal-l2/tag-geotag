# tag-geotag
情報科学実験I超高性能化課題のためのデータ構造  
[tag-pp](https://github.com/equal-l2/tag-pp)とか[tag-search](https://github.com/equal-l2/tag-search)が使う

## データ構造

```
pub struct GeoTag {
    pub time: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub domain_num: u8,
    pub url_num1: u16,
    pub url_num2: u64,
}
```

解説:  
- `time`: 2038年以前なので`i32`で格納
- `latitude`, `longitude`: そのまま
- `domain_num`: 数字1桁なので`u8`。(一文字だからといって`char`にすると32ビットになる)
- `url_num1`: 最大で数字4桁なので`u16`で足りる
- `url_num2`: 最大で16進数10桁 = 40ビットなので、`u64`

もともと110~120文字くらい(≒1000ビット)だったのが、パディング除いて248ビットになった。

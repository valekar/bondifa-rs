use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BonfidaContentError {}

error_chain! {

    errors {
        BonfidaError(response: BonfidaContentError)
     }


    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        Tungstenite(tungstenite::Error);
        TimestampError(std::time::SystemTimeError);
    }
}

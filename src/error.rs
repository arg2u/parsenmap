//! # Error handler
#[derive(Debug)]
pub struct ParsenmapError {
    pub err: String,
}

impl From<xml::reader::Error> for ParsenmapError {
    ///
    /// Generates error from `xml::reader::Error`
    ///
    /// ```
    fn from(e: xml::reader::Error) -> Self {
        ParsenmapError { err: e.to_string() }
    }
}

impl From<std::io::Error> for ParsenmapError {
    ///
    /// Generates error from `std::io::Error`
    ///
    /// ```
    fn from(e: std::io::Error) -> Self {
        ParsenmapError { err: e.to_string() }
    }
}

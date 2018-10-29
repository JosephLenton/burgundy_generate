use crate::load::Domain;
use crate::load::Url;
use extern::failure::Fail;
use extern::toml;
use extern::toml::de::Error as TOMLError;
use std::fs::File;
use std::io::BufReader;
use std::io::Error as IOError;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Api {
    pub domain: Domain,
    pub url: Vec<Url>,
}

impl Api {
    /// Returns an `Api` from the TOML file given.
    pub fn from_file(file: String) -> Result<Self, ApiError> {
        let open_file =
            File::open(&file).map_err(|err| ApiError::from_io_error(err, file.clone()))?;
        let mut buf_reader = BufReader::new(open_file);
        let mut file_contents = String::new();
        Read::read_to_string(&mut buf_reader, &mut file_contents)
            .map_err(|err| ApiError::from_io_error(err, file.clone()))?;

        Self::from_str(&file_contents)
    }

    /// Returns an `Api` parsed from an `&str`blob.
    pub fn from_str(file_str: &str) -> Result<Self, ApiError> {
        let config = toml::from_str(&file_str)?;

        Ok(config)
    }
}

#[derive(Debug, Fail)]
pub enum ApiError {
    /// This happens when we cannot read the file from disk.
    #[fail(
        display = "IO error whilst trying to read file {}\n{}",
        file,
        error
    )]
    FailedToReadFile {
        #[cause]
        error: IOError,

        file: String,
    },

    #[fail(display = "Error when trying to parse file")]
    ParseError {
        #[cause]
        error: TOMLError,
    },
}

impl ApiError {
    pub fn from_io_error(error: IOError, file: String) -> Self {
        ApiError::FailedToReadFile { error, file }
    }
}

impl From<TOMLError> for ApiError {
    fn from(error: TOMLError) -> Self {
        ApiError::ParseError { error }
    }
}

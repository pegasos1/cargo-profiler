use std::error;
use std::fmt;
use std::io::Error as ioError;


#[derive(Debug)]
/// Represents potential errors that may occur when profiling
pub enum ProfError {
    RegexError,
    InvalidProfiler,
    InvalidBinary,
    InvalidNum,
    InvalidSortMetric,
    /// Wraps a std::io::Error
    IOError(ioError),
    MisalignedData,
    CompilationError(String),
    TomlError,
}

impl fmt::Display for ProfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProfError::RegexError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mregex error -- please file a bug. In bug report, \
                        please include the original output file from profiler, e.g. from \
                        valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.txt")
            }
            ProfError::InvalidProfiler => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0minvalid profiler. cargo profiler currently \
                        supports callgrind and cachegrind.")
            }
            ProfError::InvalidBinary => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0minvalid binary. make sure binary exists.")
            }
            ProfError::InvalidNum => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0minvalid number. make sure number is a positive \
                        integer.")
            }
            ProfError::InvalidSortMetric => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0minvalid metric to sort on. available cachegrind \
                        metrics are \nir, i1mr, ilmr, dr, d1mr, dlmr, dw, d1mw, and dlmw. Check \
                        README for details on these metrics.")
            }
            ProfError::IOError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mio error: {} -- please file a bug.",
                       err)
            }
            ProfError::MisalignedData => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mmisaligned data arrays due to regex error -- \
                        please file a bug.")
            }
            ProfError::CompilationError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mfailed to compile {}. Run cargo build to get \
                        compilation error.",
                       err)
            }
            ProfError::TomlError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0merror in parsing Cargo.toml to derive package \
                        name. Make sure package name is directly under [package] tag.")
            }
        }
    }
}

impl error::Error for ProfError {
    fn description(&self) -> &str {
        match *self {
            ProfError::RegexError => "Regex error. file bug.",
            ProfError::InvalidProfiler => "Invalid Profiler.",
            ProfError::InvalidBinary => "Invalid Binary.",
            ProfError::InvalidNum => "Invalid number.",
            ProfError::InvalidSortMetric => "Invalid sort metric.",
            ProfError::MisalignedData => "Misaligned Data. File bug.",
            ProfError::CompilationError(_) => {
                "Failed to compile. Run cargo build to get compilation error."
            }
            ProfError::TomlError => "Error in parsing Cargo.toml.",
            ProfError::IOError(ref err) => err.description(),

        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ProfError::RegexError => None,
            ProfError::InvalidProfiler => None,
            ProfError::InvalidBinary => None,
            ProfError::InvalidNum => None,
            ProfError::InvalidSortMetric => None,
            ProfError::MisalignedData => None,
            ProfError::TomlError => None,
            ProfError::IOError(ref err) => Some(err),
            ProfError::CompilationError(_) => None,
        }
    }
}

impl From<ioError> for ProfError {
    fn from(err: ioError) -> ProfError {
        ProfError::IOError(err)
    }
}

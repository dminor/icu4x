use clap::{App, Arg, ArgGroup};
use icu_cldr_json_data_provider::CldrJsonDataProvider;
use icu_cldr_json_data_provider::CldrPaths;
use icu_data_provider::icu_data_key;
use icu_data_provider::iter::IterableDataProvider;
use icu_fs_data_provider::export::fs_exporter;
use icu_fs_data_provider::export::serializers;
use icu_fs_data_provider::export::FilesystemExporter;
use icu_fs_data_provider::manifest;
use icu_testdata;
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;

// #[derive(Debug)]
enum Error {
    Unsupported(&'static str),
    Export(icu_fs_data_provider::FsDataError),
    DataProvider(icu_data_provider::DataError),
    Metadata(icu_testdata::metadata::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unsupported(message) => write!(f, "Unsupported: {}", message),
            Error::Export(error) => write!(f, "{}", error),
            Error::DataProvider(error) => write!(f, "{}", error),
            Error::Metadata(error) => write!(f, "Metadata Error: {}", error),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

impl From<icu_fs_data_provider::FsDataError> for Error {
    fn from(err: icu_fs_data_provider::FsDataError) -> Error {
        Error::Export(err)
    }
}

impl From<icu_data_provider::DataError> for Error {
    fn from(err: icu_data_provider::DataError) -> Error {
        Error::DataProvider(err)
    }
}

impl From<icu_testdata::metadata::Error> for Error {
    fn from(err: icu_testdata::metadata::Error) -> Error {
        Error::Metadata(err)
    }
}

fn main() -> Result<(), Error> {
    let default_out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("json")
        .into_os_string();

    let matches = App::new("ICU4X Test Data Generator")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Export CLDR JSON into the ICU4X data schema for test data")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .help("Enable verbose logging."),
        )
        .arg(
            Arg::with_name("DRY_RUN")
                .short("n")
                .long("dry-run")
                .help("Do not touch the filesystem (consider using with -v)."),
        )
        .arg(
            Arg::with_name("OVERWRITE")
                .short("W")
                .long("overwrite")
                .help("Delete the output testdata directory before writing data."),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output data directory. Must be empty or non-existent, unless \
                    --overwrite is present, in which case the directory is deleted first. \
                    Omit this option to write data into the package tree.",
                )
                .takes_value(true)
                .default_value_os(&default_out_dir),
        )
        .get_matches();

    // TODO: Save this list of keys in a better place
    let keys = [
        icu_data_key!(plurals: cardinal@1),
        icu_data_key!(plurals: ordinal@1),
        icu_data_key!(dates: gregory@1),
    ];

    println!("{:?}", matches);

    let metadata = icu_testdata::metadata::load()?;

    println!("{:?}", metadata);

    /*
    let output_path = PathBuf::from(
        matches
            .value_of_os("OUTPUT")
            .unwrap_or_else(|| OsStr::new("/tmp/icu4x_json")),
    );

    let mut cldr_paths = CldrPaths::default();

    if let Some(path) = matches.value_of("CLDR_CORE") {
        cldr_paths.cldr_core = Ok(path.into());
    }

    if let Some(path) = matches.value_of("CLDR_DATES") {
        cldr_paths.cldr_dates = Ok(path.into());
    }

    let provider = CldrJsonDataProvider::new(&cldr_paths);

    let mut options = serializers::JsonSerializerOptions::default();
    if let Some(value) = matches.value_of("STYLE") {
        options.style = match value {
            "compact" => serializers::StyleOption::Compact,
            "pretty" => serializers::StyleOption::Pretty,
            _ => unreachable!(),
        };
    }
    let json_serializer = Box::new(serializers::JsonSerializer::new(&options));

    let mut options = fs_exporter::ExporterOptions::default();
    options.root = output_path;
    if let Some(value) = matches.value_of("ALIASING") {
        options.aliasing = match value {
            "none" => manifest::AliasOption::NoAliases,
            "symlink" => manifest::AliasOption::Symlink,
            _ => unreachable!(),
        };
    }
    if matches.is_present("OVERWRITE") {
        options.overwrite = fs_exporter::OverwriteOption::RemoveAndReplace
    }
    options.verbose = matches.is_present("VERBOSE");
    let mut exporter = FilesystemExporter::try_new(json_serializer, &options)?;

    for key in keys.iter() {
        let result = provider.export_key(key, &mut exporter);
        // Ensure flush() is called, even when the result is an error
        exporter.flush()?;
        result?;
    }
    */

    Ok(())
}

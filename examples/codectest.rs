use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[macro_use]
extern crate serde_json;

extern crate wascc_codec as codec;
use codec::Sample;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]),
    name = "codectest", 
    about = "A CLI for generating and validating codec structs")]
struct Cli {
    #[structopt(flatten)]
    command: CliCommand,
}

#[derive(Debug, Clone, StructOpt)]
enum CliCommand {
    /// Validate a codec output file
    #[structopt(name = "validate")]
    Validate(ValidateCommand),

    /// Generate a codec output file
    #[structopt(name = "generate")]
    Generate(GenerateCommand),
}

#[derive(Debug, Clone, StructOpt)]
struct GenerateCommand {
    /// Path of output file
    #[structopt(short = "p", long = "path")]
    path: String,
}

#[derive(Debug, Clone, StructOpt)]
struct ValidateCommand {
    /// Path of file to validate
    #[structopt(short = "p", long = "path")]
    path: String,
}

fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    let args = Cli::from_args();
    let cmd = args.command;

    match handle_command(cmd) {
        Ok(_) => {}
        Err(e) => {
            println!("Command line failure: {}", e);
        }
    }
    Ok(())
}

fn handle_command(cmd: CliCommand) -> Result<(), Box<dyn ::std::error::Error>> {
    match cmd {
        CliCommand::Validate(valcmd) => validate_file(&valcmd),
        CliCommand::Generate(gencmd) => generate_file(&gencmd),
    }
}

fn generate_file(cmd: &GenerateCommand) -> Result<(), Box<dyn ::std::error::Error>> {
    let output = json!({
        "version" : codec::VERSION,
        "httpserver": generate_httpserver_sample(),
        "keyvalue": generate_keyvalue_sample(),
        "blobstore": generate_blobstore_sample(),
        "messaging": generate_messaging_sample(),
        "extras": generate_extras_sample(),
        "logging": generate_logging_sample(),
        "eventstreams": generate_eventstreams_sample()
    });
    let mut buffer = File::create(&cmd.path)?;
    buffer.write_all(&serde_json::to_vec(&output).unwrap())?;
    buffer.flush()?;
    Ok(())
}

fn validate_file(cmd: &ValidateCommand) -> Result<(), Box<dyn ::std::error::Error>> {
    let mut f = File::open(&cmd.path)?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    let raw: serde_json::Value = serde_json::from_str(::std::str::from_utf8(&buffer)?)?;

    assert(
        &raw["httpserver"]["request"],
        codec::http::Request::sample(),
    )?;
    assert(
        &raw["httpserver"]["response"],
        codec::http::Response::sample(),
    )?;
    assert(
        &raw["blobstore"]["filechunk"],
        codec::blobstore::FileChunk::sample(),
    )?;
    assert(
        &raw["blobstore"]["containerlist"],
        codec::blobstore::ContainerList::sample(),
    )?;
    assert(
        &raw["extras"]["result_guid"],
        codec::extras::GeneratorResult::sample(),
    )?;
    assert(
        &raw["eventstreams"]["streamquery"],
        codec::eventstreams::StreamQuery::sample(),
    )?;
    assert(
        &raw["keyvalue"]["setrequest"],
        codec::keyvalue::SetRequest::sample(),
    )?;
    assert(
        &raw["messaging"]["requestmessage"],
        codec::messaging::RequestMessage::sample(),
    )?;
    assert(
        &raw["logging"]["writelogrequest"],
        codec::logging::WriteLogRequest::sample(),
    )?;
    println!("Valid!");
    Ok(())
}

fn generate_httpserver_sample() -> serde_json::Value {
    json!({
        "request": base64::encode(codec::serialize(codec::http::Request::sample()).unwrap()),
        "response": base64::encode(codec::serialize(codec::http::Response::sample()).unwrap())
    })
}

fn generate_keyvalue_sample() -> serde_json::Value {
    json!({
        "setrequest": base64::encode(codec::serialize(codec::keyvalue::SetRequest::sample()).unwrap())
    })
}

fn generate_blobstore_sample() -> serde_json::Value {
    json!({
        "filechunk": base64::encode(codec::serialize(codec::blobstore::FileChunk::sample()).unwrap()),
        "containerlist": base64::encode(codec::serialize(codec::blobstore::ContainerList::sample()).unwrap())
    })
}

fn generate_messaging_sample() -> serde_json::Value {
    json!({
        "requestmessage": base64::encode(codec::serialize(codec::messaging::RequestMessage::sample()).unwrap())
    })
}

fn generate_extras_sample() -> serde_json::Value {
    json!({
        "result_guid": base64::encode(codec::serialize(codec::extras::GeneratorResult::sample()).unwrap()),
    })
}

fn generate_logging_sample() -> serde_json::Value {
    json!({
        "writelogrequest": base64::encode(codec::serialize(codec::logging::WriteLogRequest::sample()).unwrap()),
    })
}

fn generate_eventstreams_sample() -> serde_json::Value {
    json!({
        "streamquery": base64::encode(codec::serialize(codec::eventstreams::StreamQuery::sample()).unwrap())
    })
}

fn assert<'de, T: Deserialize<'de> + PartialEq + std::fmt::Debug>(
    value: &serde_json::Value,
    expected: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = value.to_string().replace("\"", "");
    let bytes = base64::decode(&encoded)?;

    let val: T = codec::deserialize(&bytes)?;
    assert_eq!(val, expected);
    Ok(())
}

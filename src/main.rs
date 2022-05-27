use std::{env::set_var, path::Path};
mod tts;

fn main() -> Result<(), anyhow::Error> {
    let scorer_file;
    let (model_path, scorer_path) = initialise_app();
    let mut model = ds_transcriber::model::instance_model(
        model_path.as_ref(),
        match scorer_path {
            Some(scorer) => {
                scorer_file = scorer;
                Some(scorer_file.as_ref())
            }
            None => None,
        },
    )?;
    let i_said = ds_transcriber::transcribe(ds_transcriber::StreamSettings::default(), &mut model)?;
    println!("I said: {}", i_said);
    Ok(())
}

fn initialise_app() -> (impl AsRef<Path>, Option<impl AsRef<Path>>) {
    let model_path = "/Users/aidenlee/DeepSpeechClient/deepspeech-0.9.0-models.pbmm";
    let scorer_path = "/Users/aidenlee/DeepSpeechClient/deepspeech-0.9.0-models.scorer";
    let native_client = "/Users/aidenlee/DeepSpeechClient/";
    set_var("LD_LIBRARY_PATH", native_client);
    set_var("LIBRARY_PATH", native_client);
    (model_path.to_owned(), Option::Some(scorer_path))
}

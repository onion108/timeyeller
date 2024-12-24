use std::{env::current_exe, fs::exists, io::Cursor, path::{Path, PathBuf}};

use chrono::Timelike;
use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::{buffer::SamplesBuffer, Decoder, OutputStream, Sink};


static ALARM_WAV: &'static [u8] = include_bytes!("../resources/alarm.wav");

fn find_resource(path: &str) -> String {
    if exists("resources").unwrap() {
        let mut pathbuf = PathBuf::from("resources");
        pathbuf.push(path);
        pathbuf.to_string_lossy().into()
    } else {
        let exe_path = current_exe().unwrap();
        let mut pathbuf = PathBuf::from(exe_path);
        pathbuf.push("..");
        pathbuf.push("resources");
        pathbuf.push(path);
        pathbuf.to_string_lossy().into()
    }
}

fn generate_text() -> String {
    let current_time = chrono::offset::Local::now().time();
    let hour = current_time.hour();
    let minutes = current_time.minute();
    let text = format!("{}点{}", hour, match minutes {
        0 => "整".into(),
        30 => "半".into(),
        x => format!("{}分", x)
    });
    format!("现在是{}", text)
}

fn play_the_fucking_sound(synth: &PiperSpeechSynthesizer) {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Ouch! Where is my mouth and throat! I can't make any sound.");
    let sink = Sink::try_new(&stream_handle).expect("Cannot think about the audio track, please fucking teach me that ok?");

    let alarm_sound = Cursor::new(ALARM_WAV);
    let source = Decoder::new(alarm_sound).expect("What the fuck there is an error in your audio file! Holy shit.");
    sink.append(source);

    let mut samples = Vec::new();
    let audio = synth.synthesize_parallel(generate_text(), None).unwrap();
    for result in audio {
        samples.append(&mut result.unwrap().into_vec());
    }
    let buf = SamplesBuffer::new(1, 22050, samples);
    sink.append(buf);

    sink.sleep_until_end();
}

fn main() {
    let path = find_resource("zh_CN-huayan-medium.onnx.json");
    let model = piper_rs::from_config_path(&Path::new(&path)).expect("sorry i don't know how to speak human language bro");
    model.set_speaker(0);
    let synth = PiperSpeechSynthesizer::new(model).unwrap();

    loop {
        let now = chrono::offset::Local::now().time();
        match now.minute() {
            0 => {
                play_the_fucking_sound(&synth);
            }
            30 => {
                play_the_fucking_sound(&synth);
            }
            _ => {}
        }
    }
}


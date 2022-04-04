use std::time::Duration;

use fltk::{prelude::*, *};
use rodio::{OutputStream, Sink, source::SineWave, Source};
mod ui;

fn main() {
    let app = app::App::default();
    let mut ui = ui::UserInterface::make_window();
    ui.but.set_callback(move |_| {
        println!("Sync!");

				let (_stream, stream_handle) = OutputStream::try_default().unwrap();
				let sink = Sink::try_new(&stream_handle).unwrap();

				let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.9);
				sink.append(source);

				sink.sleep_until_end();
    });
    app.run().unwrap();
}
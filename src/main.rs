use std::time::Duration;

use fltk::{prelude::*, *}; // Library for GUI
use rodio::{OutputStream, Sink, source::SineWave, Source}; // Library for audio playback
mod ui; // reference to ui.rs

fn main() {
	let app = app::App::default();
	let mut ui = ui::UserInterface::make_window(); // create a window

	//function that plays when the button is pressed
	ui.but.set_callback(move |_| {
		println!("Sync!");

		// Get a output stream handle to the default physical sound device
		let (_stream, stream_handle) = OutputStream::try_default().unwrap();
		let sink = Sink::try_new(&stream_handle).unwrap();

		//generate sine wave sound
		let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.9);
		sink.append(source);

		// The sound plays in a separate thread. This call will block the current thread until the sink
		// has finished playing all its queued sounds.
		sink.sleep_until_end();
	});
	app.run().unwrap();
}
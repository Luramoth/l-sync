use std::{time::Duration};

use fltk::{prelude::*, *}; // Library for GUI
use rodio::{OutputStream, Sink, source::SineWave, Source}; // Library for audio playback
mod ui; // reference to ui.rs

/*  l-sync a content creator synchronization tool
 Copyright (C) 2022  Luramoth

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU Affero General Public License as
 published by the Free Software Foundation, either version 3 of the
 License, or (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU Affero General Public License for more details.

 You should have received a copy of the GNU Affero General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>. */


fn main() {
	let app = app::App::default();
	let mut ui = ui::UserInterface::make_window(); // create a window

	let icon = image::PngImage::load("l-sync_icon.png").unwrap();

	ui.wind.set_icon(Some(icon));


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
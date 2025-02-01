/**
 * Copyright 2025 FlacSy
 *
 * Licensed under the FlacSy Project License, Version 1.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License by contacting the author at
 *
 *     flacsy.tw@gmail.com
 *
 * This software is provided "as is", without any warranties, express or implied.
 * The author is not responsible for any damages or losses arising from the use of this software.
 * The License governs the permissions and restrictions related to the use, modification,
 * and distribution of this software.
 *
 * Commercial use is only permitted with prior written consent from the author.
 */


use std::time::Duration;


#[derive(Clone)]
pub struct Track {
    pub title: String,
    // pub artist: Option<String>,
    // pub album: Option<String>,
    pub file_data: Vec<u8>,
    // pub duration: Option<Duration>,
    pub file_path: Option<String>,
}

impl Track {
    pub fn new(title: String, file_data: Vec<u8>, artist: Option<String>, album: Option<String>, duration: Option<Duration>, file_path: Option<String>) -> Self {
        println!("[DEBUG] Creating new track: title={}, artist={:?}, album={:?}, duration={:?}, file_path={:?}", title, artist, album, duration, file_path);
        Track {
            title,
            // artist,
            // album,
            file_data,
            // duration,
            file_path,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_file_data(&self) -> &Vec<u8> {
        &self.file_data
    }

    pub fn get_file_path(&self) -> &Option<String> {
        &self.file_path
    }
}
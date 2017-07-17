use std::io;
use std::fs::File;

extern crate bitstream_io;
extern crate chrono;

mod ebml;
mod ids;

use chrono::{DateTime, Duration};
use chrono::offset::Utc;

#[derive(Debug)]
pub struct MKV {
    pub info: Info,
    pub tracks: Vec<Track>,
    //pub chapters: Vec<Chapter>,
    //pub tags: Vec<Tag>
}

#[derive(Debug)]
pub enum ReadMKVError {
    Io(io::Error)
}

impl MKV {
    pub fn new() -> MKV {
        MKV{info: Info::new(),
            tracks: Vec::new()}
    }

    pub fn open(mut file: File) -> Result<MKV,ReadMKVError> {
        use std::io::Seek;
        use std::io::SeekFrom;

        let mut mkv = MKV::new();

        // look for first Segment in stream
        /*FIXME - clean this up*/
        let (mut id_0, mut size_0, _) =
            ebml::read_element_id_size(&mut file).unwrap();
        while id_0 != ids::SEGMENT {
            file.seek(SeekFrom::Current(size_0 as i64)).map(|_| ()).unwrap();
            let (id, size, _) = ebml::read_element_id_size(&mut file).unwrap();
            id_0 = id;
            size_0 = size;
        }

        // pull out useful pieces from Segment
        while size_0 > 0 {
            let (id_1, size_1, len) =
                ebml::read_element_id_size(&mut file).unwrap();
            /*FIXME - implement extraction*/
            match id_1 {
                ids::INFO => {
                    mkv.info = Info::parse(&mut file, size_1)?;
                }
                ids::TRACKS => {
                    mkv.tracks = Track::parse(&mut file, size_1)?;
                }
                _ => {
                    println!("level1 : {:X} {}", id_1, size_1);
                    file.seek(SeekFrom::Current(size_1 as i64))
                        .map(|_| ())
                        .unwrap();
                }
            }
            size_0 -= len;
            size_0 -= size_1;
        }

        Ok(mkv)
    }
}

#[derive(Debug)]
pub struct Info {
    pub title: Option<String>,
    pub duration: Option<f64>,
    pub date_utc: Option<DateTime<Utc>>,
    pub muxing_app: Option<String>,
    pub writing_app: Option<String>
}

impl Info {
    pub fn new() -> Info {
        Info{title: None,
             duration: None,
             date_utc: None,
             muxing_app: None,
             writing_app: None}
    }

    pub fn parse(r: &mut io::Read, mut size: u64) -> Result<Info,ReadMKVError> {
        let mut info = Info::new();
        let mut timecode_scale = None;
        let mut duration = None;

        while size > 0 {
            /*FIXME - improve error handling*/
            let (i, s, len) = ebml::read_element_id_size(r).unwrap();
            match i {
                ids::TITLE => {
                    info.title = Some(ebml::read_utf8(r, s).unwrap());
                }
                ids::TIMECODESCALE => {
                    timecode_scale = Some(ebml::read_uint(r, s).unwrap());
                }
                ids::DURATION => {
                    duration = Some(ebml::read_float(r, s).unwrap());
                }
                ids::DATEUTC => {
                    info.date_utc = Some(ebml::read_date(r, s).unwrap());
                }
                ids::MUXINGAPP => {
                    info.muxing_app = Some(ebml::read_utf8(r, s).unwrap());
                }
                ids::WRITINGAPP => {
                    info.writing_app = Some(ebml::read_utf8(r, s).unwrap());
                }
                _ => {ebml::read_bin(r, s).unwrap();}
            }
            size -= len;
            size -= s;
        }

        if let Some(d) = duration {
            if let Some(t) = timecode_scale {
                info.duration = Some((d * t as f64) / 1_000_000_000.0)
            }
        }
        Ok(info)
    }
}

#[derive(Debug)]
pub struct Video {
    pixel_width: u64,
    pixel_height: u64,
    display_width: u64,
    display_height: u64
}

impl Video {
    fn new() -> Video {
        Video{pixel_width: 0,
              pixel_height: 0,
              display_width: 0,
              display_height: 0}
    }

    fn parse(r: &mut io::Read, mut size: u64) -> Result<Video,ReadMKVError> {
        let mut video = Video::new();

        while size > 0 {
            let (i, s, len) = ebml::read_element_id_size(r).unwrap();

            match i {
                ids::PIXELWIDTH => {
                    video.pixel_width = ebml::read_uint(r, s).unwrap();
                }
                ids::PIXELHEIGHT => {
                    video.pixel_height = ebml::read_uint(r, s).unwrap();
                }
                ids::DISPLAYWIDTH => {
                    video.display_width = ebml::read_uint(r, s).unwrap();
                }
                ids::DISPLAYHEIGHT => {
                    video.display_height = ebml::read_uint(r, s).unwrap();
                }
                _ => {
                    println!("video : {:X} {}", i, s);
                    let _ = ebml::read_bin(r, s).unwrap();
                }
            }

            size -= len;
            size -= s;
        }

        Ok(video)
    }
}

#[derive(Debug)]
pub struct Audio {
    sample_rate: f64,
    channels: u64,
    bit_depth: u64
}

impl Audio {
    fn new() -> Audio {
        Audio{sample_rate: 0.0,
              channels: 0,
              bit_depth: 0}
    }

    fn parse(r: &mut io::Read, mut size: u64) -> Result<Audio,ReadMKVError> {
        let mut audio = Audio::new();

        while size > 0 {
            let (i, s, len) = ebml::read_element_id_size(r).unwrap();

            match i {
                ids::SAMPLINGFREQUENCY => {
                    audio.sample_rate = ebml::read_float(r, s).unwrap();
                }
                ids::CHANNELS => {
                    audio.channels = ebml::read_uint(r, s).unwrap();
                }
                ids::BITDEPTH => {
                    audio.bit_depth = ebml::read_uint(r, s).unwrap();
                }
                _ => {
                    println!("audio track : {:X} {}", i, s);
                    let _ = ebml::read_bin(r, s).unwrap();
                }
            }

            size -= len;
            size -= s;
        }

        Ok(audio)
    }
}

#[derive(Debug)]
pub enum Settings {
    None,
    Video(Video),
    Audio(Audio)
}

#[derive(Debug)]
pub struct Track {
    number: u64,
    uid: u64,
    tracktype: u64, /*FIXME - make enum?*/
    enabled: bool,
    default: bool,
    forced: bool,
    interlaced: bool,
    defaultduration: Duration,
    offset: i64,
    name: String,
    language: String,
    codec_id: String,
    codec_name: String,
    settings: Settings
}

impl Track {
    fn new() -> Track {
        Track{number: 0,
              uid: 0,
              tracktype: 0,
              enabled: true,
              default: true,
              forced: false,
              interlaced: true,
              defaultduration: Duration::nanoseconds(0),
              offset: 0,
              name: String::new(),
              language: String::new(),
              codec_name: String::new(),
              codec_id: String::new(),
              settings: Settings::None}
    }

    pub fn parse(r: &mut io::Read, mut size: u64) ->
        Result<Vec<Track>,ReadMKVError> {
        let mut tracks = Vec::new();

        while size > 0 {
            let (i, s, len) = ebml::read_element_id_size(r).unwrap();
            if i == ids::TRACKENTRY {
                tracks.push(Track::parse_entry(r, s)?);
            } else {
                let _ = ebml::read_bin(r, s).unwrap();
            }

            size -= len;
            size -= s;
        }
        Ok(tracks)
    }

    fn parse_entry(r: &mut io::Read, mut size: u64) ->
        Result<Track,ReadMKVError> {
        let mut track = Track::new();
        while size > 0 {
            let (i, s, len) = ebml::read_element_id_size(r).unwrap();

            match i {
                ids::TRACKNUMBER => {
                    track.number = ebml::read_uint(r, s).unwrap();
                }
                ids::TRACKUID => {
                    track.uid = ebml::read_uint(r, s).unwrap();
                }
                ids::TRACKTYPE => {
                    track.tracktype = ebml::read_uint(r, s).unwrap();
                }
                ids::FLAGENABLED => {
                    track.enabled = ebml::read_uint(r, s).unwrap() != 0;
                }
                ids::FLAGDEFAULT => {
                    track.default = ebml::read_uint(r, s).unwrap() != 0;
                }
                ids::FLAGFORCED => {
                    track.forced = ebml::read_uint(r, s).unwrap() != 0;
                }
                ids::FLAGLACING => {
                    track.interlaced = ebml::read_uint(r, s).unwrap() != 0;
                }
                ids::DEFAULTDURATION => {
                    track.defaultduration =
                        Duration::nanoseconds(
                            ebml::read_uint(r, s).unwrap() as i64);
                }
                ids::TRACKOFFSET => {
                    track.offset = ebml::read_int(r, s).unwrap();
                }
                ids::NAME => {
                    track.name = ebml::read_utf8(r, s).unwrap();
                }
                ids::LANGUAGE => {
                    track.language = ebml::read_string(r, s).unwrap()
                }
                ids::CODEC_ID => {
                    track.codec_id = ebml::read_string(r, s).unwrap();
                }
                ids::CODEC_NAME => {
                    track.codec_name = ebml::read_utf8(r, s).unwrap();
                }
                ids::VIDEO => {
                    track.settings = Settings::Video(Video::parse(r, s)?);
                }
                ids::AUDIO => {
                    track.settings = Settings::Audio(Audio::parse(r, s)?);
                }
                _ => {
                    println!("track entry : {:X} {}", i, s);
                    let _ = ebml::read_bin(r, s).unwrap();
                }
            }

            size -= len;
            size -= s;
        }
        Ok(track)
    }
}

//pub struct Chapter {
//    /*FIXME*/
//}
//
//pub struct Tag {
//    /*FIXME*/
//}

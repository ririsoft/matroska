// Copyright 2017-2022 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub const SEGMENT: u32 = 0x1853_8067;
pub const SEEKHEAD: u32 = 0x114D_9B74;
pub const SEEK: u32 = 0x4DBB;
pub const SEEKID: u32 = 0x53AB;
pub const SEEKPOSITION: u32 = 0x53AC;
pub const INFO: u32 = 0x1549_A966;
pub const SEGMENTUID: u32 = 0x73A4;
pub const PREVUID: u32 = 0x3C_B923;
pub const NEXTUID: u32 = 0x3E_B923;
pub const SEGMENTFAMILY: u32 = 0x4444;
pub const TITLE: u32 = 0x7BA9;
pub const MUXINGAPP: u32 = 0x4D80;
pub const WRITINGAPP: u32 = 0x5741;
pub const DATEUTC: u32 = 0x4461;
pub const TIMECODESCALE: u32 = 0x2A_D7B1;
pub const DURATION: u32 = 0x4489;
pub const TRACKS: u32 = 0x1654_AE6B;
pub const TRACKENTRY: u32 = 0xAE;
pub const TRACKNUMBER: u32 = 0xD7;
pub const TRACKUID: u32 = 0x73C5;
pub const TRACKTYPE: u32 = 0x83;
pub const FLAGENABLED: u32 = 0xB9;
pub const FLAGDEFAULT: u32 = 0x88;
pub const FLAGFORCED: u32 = 0x55AA;
pub const FLAGHEARINGIMPAIRED: u32 = 0x55AB;
pub const FLAGVISUALIMPAIRED: u32 = 0x55AC;
pub const FLAGTEXTDESCRIPTIONS: u32 = 0x55AD;
pub const FLAGORIGINAL: u32 = 0x55AE;
pub const FLAGCOMMENTARY: u32 = 0x55AF;
pub const FLAGLACING: u32 = 0x9C;
pub const DEFAULTDURATION: u32 = 0x23_E383;
pub const NAME: u32 = 0x536E;
pub const LANGUAGE: u32 = 0x22_B59C;
pub const LANGUAGE_IETF: u32 = 0x22_B59D;
pub const CODEC_ID: u32 = 0x86;
pub const CODEC_PRIVATE: u32 = 0x63A2;
pub const CODEC_NAME: u32 = 0x25_8688;
pub const VIDEO: u32 = 0xE0;
pub const PIXELWIDTH: u32 = 0xB0;
pub const PIXELHEIGHT: u32 = 0xBA;
pub const DISPLAYWIDTH: u32 = 0x54B0;
pub const DISPLAYHEIGHT: u32 = 0x54BA;
pub const INTERLACED: u32 = 0x9A;
pub const STEREOMODE: u32 = 0x53B8;
pub const AUDIO: u32 = 0xE1;
pub const SAMPLINGFREQUENCY: u32 = 0xB5;
pub const CHANNELS: u32 = 0x9F;
pub const BITDEPTH: u32 = 0x6264;
pub const ATTACHMENTS: u32 = 0x1941_A469;
pub const ATTACHEDFILE: u32 = 0x61A7;
pub const FILEDESCRIPTION: u32 = 0x467E;
pub const FILENAME: u32 = 0x466E;
pub const FILEMIMETYPE: u32 = 0x4660;
pub const FILEDATA: u32 = 0x465C;
pub const CHAPTERS: u32 = 0x1043_A770;
pub const EDITIONENTRY: u32 = 0x45B9;
pub const EDITIONUID: u32 = 0x45BC;
pub const EDITIONFLAGHIDDEN: u32 = 0x45BD;
pub const EDITIONFLAGDEFAULT: u32 = 0x45DB;
pub const EDITIONFLAGORDERED: u32 = 0x45DD;
pub const CHAPTERATOM: u32 = 0xB6;
pub const CHAPTERUID: u32 = 0x73C4;
pub const CHAPTERTIMESTART: u32 = 0x91;
pub const CHAPTERTIMEEND: u32 = 0x92;
pub const CHAPTERFLAGHIDDEN: u32 = 0x98;
pub const CHAPTERFLAGENABLED: u32 = 0x4598;
pub const CHAPTERSEGMENTUID: u32 = 0x6E67;
pub const CHAPTERSEGMENTEDITIONUID: u32 = 0x6EBC;
pub const CHAPTERDISPLAY: u32 = 0x80;
pub const CHAPSTRING: u32 = 0x85;
pub const CHAPLANGUAGE: u32 = 0x437C;
pub const CHAPLANGUAGE_IETF: u32 = 0x437D;
pub const TAGS: u32 = 0x1254_C367;
pub const TAG: u32 = 0x7373;
pub const TARGETS: u32 = 0x63C0;
pub const TARGETTYPEVALUE: u32 = 0x68CA;
pub const TARGETTYPE: u32 = 0x63CA;
pub const TAG_TRACK_UID: u32 = 0x63C5;
pub const TAG_EDITION_UID: u32 = 0x63C9;
pub const TAG_CHAPTER_UID: u32 = 0x63C4;
pub const TAG_ATTACHMENT_UID: u32 = 0x63C6;
pub const SIMPLETAG: u32 = 0x67C8;
pub const TAGNAME: u32 = 0x45A3;
pub const TAGLANGUAGE: u32 = 0x447A;
pub const TAGLANGUAGE_IETF: u32 = 0x447B;
pub const TAGDEFAULT: u32 = 0x4484;
pub const TAGSTRING: u32 = 0x4487;
pub const TAGBINARY: u32 = 0x4485;

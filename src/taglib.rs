/*!
Rustic wrapper around the taglib abstract C api.

Example:

~~~rust 
use taglib;

let mymusic = Path::new("my.ogg");
let mytag = Tag::new(music);
let mytitle = mytag.title();
~~~

*/

#[link(name       = "taglib",
       vers       = "1.0",
       package_id = "taglib",
       author     = "Craig Joly")];

#[comment = "Wrapper for taglib."];
#[crate_type = "lib"];


use std::str;
use tag_c::TagLib_File;
mod tag_c;


pub enum Properties {
  length,
  bitrate,
  samplerate,
  channels
}

pub enum Type {
  MPEG      = 0,
  OggVorbis = 1,
  FLAC      = 2,
  MPC       = 3,
  OggFlac   = 4,
  WavPack   = 5,
  Speex     = 6,
  TrueAudio = 7,
  MP4       = 8,
  ASF       = 9
}

pub enum ID3V2_Encoding {
  Latin1  = 0,
  UTF16   = 1,
  UTF16BE = 2,
  UTF8    = 3
}

/**
 * By default all strings coming into or out of TagLib's C API are in UTF8.
 * However, it may be desirable for TagLib to operate on Latin1 (ISO-8859-1)
 * strings in which case this should be set to false.
 **/
pub fn setUnicode(unicode: bool) {
  unsafe {
    match unicode {
      true  => tag_c::taglib_set_strings_unicode(1),
      _     => tag_c::taglib_set_strings_unicode(0)
    }
  }
}

/// Set the default encoding for ID3v2 frames that are written to tags.  Available encodings: Latin1, UTF16, UTF16BE, UTF8.
pub fn setidv3v2DefaultEncoding(coding: ID3V2_Encoding) {
  unsafe {
    tag_c::taglib_id3v2_set_default_text_encoding(coding as u32);
  }
}

/// Tags are managed through the Tag type.  It "owns" the C pointers and will automatically deallocate them when the tag is dropped.
pub struct Tag {
  priv file: *tag_c::TagLib_File,
  priv tag:  *tag_c::TagLib_Tag,
  priv properties: *tag_c::TagLib_AudioProperties
}

impl Tag {
/// Tag constructor.
  pub fn new(file: Path) -> Tag {
    assert!(file.exists());
    unsafe {
      let filePtr = file.to_c_str().unwrap();
      let file = tag_c::taglib_file_new(filePtr);
      assert!(tag_c::taglib_file_is_valid(file) != 0);
      Tag { file : file, 
            tag  : tag_c::taglib_file_tag(file),
            properties : tag_c::taglib_file_audioproperties(file)
      }
    }
  }

/// Tag constructor when the file type must be specified. Valid types are MPEG, OggVorbis, FLAC, MPC, OggFlac, WavPack, Speex, TrueAudio, MP4, ASF.
  pub fn new_by_type(&self, file: Path, _type: Type) -> Tag{
    assert!(file.exists());
    unsafe {
      let filePtr = file.to_c_str().unwrap();
      let file = tag_c::taglib_file_new_type(filePtr, _type as u32);
      assert!(tag_c::taglib_file_is_valid(file) != 0);
      Tag { file : file,
            tag  : tag_c::taglib_file_tag(file),
            properties : tag_c::taglib_file_audioproperties(file)
      }
    }
  }

/// Returns the title
  pub fn title(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_title(self.tag))
    }
  }

/// Returns the artist
  pub fn artist(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_artist(self.tag))
    }
  }

/// Returns the comment field
  pub fn comment(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_comment(self.tag))
    }
  }
  
/// Returns the genre
  pub fn genre(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_genre(self.tag))
    }
  }

/// Returns the year
  pub fn year(&self) -> u32 {
    unsafe {
      tag_c::taglib_tag_year(self.tag)
    }
  }

/// Returns the track number
  pub fn track(&self) -> u32 {
    unsafe {
      tag_c::taglib_tag_track(self.tag)
    }
  }

/// Returns the requested audio property.  Valid properties are length, bitrate, samplerate, channels.
  pub fn properties(&self, prop: Properties) -> i32 {
    unsafe {
      match prop {
        length      => tag_c::taglib_audioproperties_length(self.properties),
        bitrate     => tag_c::taglib_audioproperties_bitrate(self.properties),
        samplerate  => tag_c::taglib_audioproperties_samplerate(self.properties),
        channels    => tag_c::taglib_audioproperties_channels(self.properties),
      }
    }
  }

/// Set the title
  pub fn setTitle(&self, title: &str) {
    unsafe {
      let titlePtr = title.to_c_str().unwrap();
      tag_c::taglib_tag_set_title(self.tag, titlePtr);
    }
  }

/// Set the artist
  pub fn setArtist(&self, artist: &str) {
    unsafe {
      let artistPtr = artist.to_c_str().unwrap();
      tag_c::taglib_tag_set_artist(self.tag, artistPtr);
    }
  }

/// Set the album name
  pub fn setAlbum(&self, album: &str) {
    unsafe {
      let albumPtr = album.to_c_str().unwrap();
      tag_c::taglib_tag_set_album(self.tag, albumPtr);
    }
  }

/// Set the comment field
  pub fn setComment(&self, comment: &str) {
    unsafe {
      let commentPtr = comment.to_c_str().unwrap();
      tag_c::taglib_tag_set_comment(self.tag, commentPtr);
    }
  }

/// Set the year
  pub fn setYear(&self, year: u32) {
    unsafe {
      tag_c::taglib_tag_set_year(self.tag, year);
    }
  }

/// Set the track number
  pub fn setTrack(&self, track: u32) {
    unsafe {
      tag_c::taglib_tag_set_year(self.tag, track);
    }
  }

}

#[doc(hidden)]
#[unsafe_destructor]
impl Drop for Tag {
  fn drop(&mut self) {
    unsafe {
      tag_c::taglib_tag_free_strings();
      tag_c::taglib_file_free(self.file);
    }
  }
}

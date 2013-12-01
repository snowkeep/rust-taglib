#[link(name   = "taglib",
       vers   = "1.0",
       author = "Craig Joly")];

#[comment = "Wrapper for taglib."];
#[crate_type = "lib"];

use std::str;
use tag_c::TagLib_File;
mod tag_c;

// available tags
pub struct Tag {
  priv file: tag_c::TagLib_File,
  priv tag: tag_c::TagLib_Tag,
  priv audioProperties: tag_c::TagLib_AudioProperties
}

enum Properties {
  length,
  bitrate,
  samplerate,
  channels
}

enum Type {
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

enum ID3V2_Encoding {
  Latin1  = 0,
  UTF16   = 1,
  UTF16BE = 2,
  UTF8    = 3
}

fn set_unicode(unicode: bool) {
  unsafe {
    match unicode {
      true  => tag_c::taglib_set_strings_unicode(1),
      _     => tag_c::taglib_set_strings_unicode(0)
    }
  }
}

impl Tag {
  fn new(&self, file: Path) {
    assert!(file.exists());
    unsafe {
      let filePtr = file.to_c_str().unwrap();

      self.file = tag_c::taglib_file_new(filePtr);
      assert!(tag_c::taglib_file_is_valid(self.file));
      self.tag  = tag_c::taglib_file_tag(self.file);
    }
  }

  fn new_by_type(&self, file: Path, _type: Type) {
    assert!(file.exists());
    unsafe {
      let filePtr = file.to_c_str().unwrap();

      self.file = tag_c::taglib_file_new_type(filePtr, _type);
      assert!(tag_c::taglib_file_is_valid(self.file));
      self.tag  = tag_c::taglib_file_tag(self.file);
    }
  }


  fn title(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_title(self.tag))
    }
  }

  fn artist(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_artist(self.tag))
    }
  }

  fn comment(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_comment(self.tag))
    }
  }
  
  fn genre(&self) -> ~str {
    unsafe {
      str::raw::from_c_str(tag_c::taglib_tag_genre(self.tag))
    }
  }

  fn year(&self) -> ~i8 {
    unsafe {
      tag_c::taglib_tag_year(self.tag) as ~i8
    }
  }

  fn track(&self) -> ~i8 {
    unsafe {
      tag_c::taglib_tag_track(self.tag) as ~i8
    }
  }

}


#[unsafe_destructor]
impl Drop for Tag {
  fn drop(&mut self) {
    unsafe {
      tag_c::taglib_tag_free_strings();
      tag_c::taglib_file_free(self.file);
    }
  }
}

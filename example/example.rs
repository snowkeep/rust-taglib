extern mod taglib;

use std::os;

//use taglib::{Tag,AudioProperties};

#[main]
fn main() {
  let args : ~[~str] = os::args();
  let myMusic = Path::new(args[1]);
  let myTag = taglib::Tag::new(myMusic);
  println(format!("Title:  {}", myTag.title()));
  println(format!("Artist: {}", myTag.artist()));
  println(format!("Year:   {}", myTag.year()));
  println(format!("Length: {}s", myTag.properties(taglib::length)));
}

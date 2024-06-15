use super::FixedVec;
use super::{RawPrefix, RawTags, Span};

/// `@a=a;b=b;c= :<rest>`
#[inline(always)]
pub fn parse_tags(src: &str, pos: &mut usize) -> RawTags {
  if !src[*pos..].starts_with('@') {
    return RawTags::default();
  }

  let start = *pos + 1;
  let mut tags = FixedVec::new();
  let mut key = Span::from(start..0);
  let mut value = Span::from(0..0);
  let mut end = 0;

  let bytes = src.as_bytes();
  for i in start..bytes.len() {
    match unsafe { *bytes.get_unchecked(i) } {
      b' ' if unsafe { *bytes.get_unchecked(i + 1) } == b':' => {
        value.end = i as u32;
        if key.end - key.start > 0 {
          whitelist.maybe_insert(src, &mut tags, key, value);
        }
        end = i + 1;
        break;
      }
      b'=' if value.end <= key.end => {
        let i = i as u32;

        key.end = i;
        value.start = i + 1;
        value.end = i + 1;
      }
      b';' => {
        let i = i as u32;

        value.end = i;
        whitelist.maybe_insert(src, &mut tags, key, value);
        key.start = i + 1;
        key.end = i + 1;
      }
      _ => {}
    }
  }

  *pos = end;

  tags
}

#[cfg(test)]
mod tests {
  use crate::irc::{whitelist_insert_all, Tag};

  use super::*;

  #[test]
  fn tags() {
    let data = "@login=test;id=asdf :<rest>";
    let mut pos = 0;

    let tags = parse_tags(data, &mut pos, &Whitelist::<16, _>(whitelist_insert_all));
    assert_eq!(pos, 20);
    let tags = tags
      .into_iter()
      .map(|tag| tag.get(data))
      .collect::<Vec<_>>();
    assert_eq!(&tags[..], &[(Tag::Login, "test"), (Tag::Id, "asdf")])
  }

  #[test]
  fn whitelist_tags() {
    let data = "@login=test;id=asdf :<rest>";
    let mut pos = 0;

    let tags = parse_tags(data, &mut pos, &whitelist!(Login));
    assert_eq!(pos, 20);
    let tags = tags
      .into_iter()
      .map(|tag| tag.get(data))
      .collect::<Vec<_>>();
    assert_eq!(&tags[..], &[(Tag::Login, "test")])
  }

  #[test]
  fn prefix() {
    let data = ":nick!user@host <rest>";
    let mut pos = 0;
    let prefix = parse_prefix(data, &mut pos);
    assert_eq!(pos, 16);
    let prefix = prefix.unwrap();
    assert_eq!(prefix.nick.unwrap().get(data), "nick");
    assert_eq!(prefix.user.unwrap().get(data), "user");
    assert_eq!(prefix.host.get(data), "host");
    assert_eq!(&data[pos..], "<rest>");

    let data = ":nick@host <rest>";
    let mut pos = 0;
    let prefix = parse_prefix(data, &mut pos);
    assert_eq!(pos, 11);
    let prefix = prefix.unwrap();
    assert_eq!(prefix.nick.unwrap().get(data), "nick");
    assert!(prefix.user.is_none());
    assert_eq!(prefix.host.get(data), "host");
    assert_eq!(&data[pos..], "<rest>");

    let data = ":host <rest>";
    let mut pos = 0;
    let prefix = parse_prefix(data, &mut pos);
    assert_eq!(pos, 6);
    let prefix = prefix.unwrap();
    assert!(prefix.nick.is_none());
    assert!(prefix.user.is_none());
    assert_eq!(prefix.host.get(data), "host");
    assert_eq!(&data[pos..], "<rest>");
  }
}

use crate::parameters::RouteParameters;
use std::collections::HashMap;
use std::str::Chars;

/// Represents a path in HTTP sense (starting from `/`)
/// This path is internal to the crate, and encapsulates the path matching
/// logic of a route.
#[derive(Debug)]
pub(crate) struct Path {
    path: String,
}

impl Path {
    pub fn new(path: &str) -> Path {
        Path {
            path: path.to_string(),
        }
    }

    pub fn matches(&self, other_path: &str) -> Option<RouteParameters> {
        // create two pointers to each of the path strings.
        let mut self_chars = self.path.chars();
        let mut path_chars = other_path.chars();
        let mut params: HashMap<String, String> = HashMap::new();
        loop {
            let self_char = self_chars.next();
            let path_char = path_chars.next();
            match (self_char, path_char) {
                (Some(s), Some(p)) if s == ':' => {
                    // capture the current fragment
                    let (key, value) = capture_route_parameter(&mut self_chars, &mut path_chars);
                    let v = format!("{}{}", p, value);
                    params.insert(key, v);
                }
                (Some(s), Some(p)) if s != ':' && s != p => {
                    return None;
                }
                (None, None) => {
                    break;
                }
                _ => {}
            }
        }
        Some(RouteParameters::new(params))
    }
}

fn capture_route_parameter(route: &mut Chars, path: &mut Chars) -> (String, String) {
    let key: String = route.take_while(|c| c != &'/').collect();
    let value: String = path.take_while(|c| c != &'/').collect();
    (key, value)
}

// We only impl PartialEq for Path when we're compiling in test configuration so
// we can use assert_eq!(path_1, path_2)
#[cfg(test)]
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_path_matches_path() {
        let path = Path::new("/foo/bar");
        {
            let matches = path.matches("/foo");
            assert!(matches.is_none());
        }
        {
            let matches = path.matches("/foo/bar").unwrap();
            assert_eq!(matches.len(), 0);
        }
        {
            let matches = path.matches("/foo/bar/baz");
            assert!(matches.is_none());
        }
    }

    #[test]
    fn test_parametric_path_with_long_variable_matches_path() {
        let path = Path::new("/foo/:foooooooooooooooooooooooooooooid");
        {
            let matches = path.matches("/foo");
            assert!(matches.is_none());
        }
        {
            let params = path.matches("/foo/bar").unwrap();
            assert_eq!(
                params.get("foooooooooooooooooooooooooooooid").unwrap(),
                &"bar".to_string()
            );
        }
        {
            let params = path
                .matches("/foo/barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr")
                .unwrap();
            assert_eq!(
                params.get("foooooooooooooooooooooooooooooid").unwrap(),
                &"barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr".to_string()
            );
        }
        {
            let matches = path.matches("/foo/bar/baz");
            assert!(matches.is_none());
        }
    }

    #[test]
    fn test_parametric_path_with_short_variable_matches_path() {
        let path = Path::new("/foo/:a");
        {
            let matches = path.matches("/foo");
            assert!(matches.is_none());
        }
        {
            let params = path.matches("/foo/bar").unwrap();
            assert_eq!(params.get("a").unwrap(), &"bar".to_string());
        }
        {
            let params = path
                .matches("/foo/barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr")
                .unwrap();
            assert_eq!(
                params.get("a").unwrap(),
                &"barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr".to_string()
            );
        }
        {
            let matches = path.matches("/foo/bar/baz");
            assert!(matches.is_none());
        }
    }

    #[test]
    fn test_parametric_path_with_multiple_variables_matches_path() {
        let path = Path::new("/users/:user_id/friend/:friend_id/group/:group_id");
        let params = path.matches("/users/1/friend/2/group/77").unwrap();
        assert_eq!(params.get("user_id").unwrap(), &"1".to_string());
        assert_eq!(params.get("friend_id").unwrap(), &"2".to_string());
        assert_eq!(params.get("group_id").unwrap(), &"77".to_string());
    }

    #[test]
    fn test_parametric_path_with_only_variables() {
        let path = Path::new("/:one/:two/:three");
        {
            let matches = path.matches("/1/2/3").unwrap();
            assert_eq!(matches.get("one").unwrap(), &"1".to_string());
            assert_eq!(matches.get("two").unwrap(), &"2".to_string());
            assert_eq!(matches.get("three").unwrap(), &"3".to_string());
        }
        {
            let matches = path.matches("/hello/howdie/hey").unwrap();
            assert_eq!(matches.get("one").unwrap(), &"hello".to_string());
            assert_eq!(matches.get("two").unwrap(), &"howdie".to_string());
            assert_eq!(matches.get("three").unwrap(), &"hey".to_string());
        }
        {
            let matches = path.matches("/hello/hello/hello").unwrap();
            assert_eq!(matches.get("one").unwrap(), &"hello".to_string());
            assert_eq!(matches.get("two").unwrap(), &"hello".to_string());
            assert_eq!(matches.get("three").unwrap(), &"hello".to_string());
        }
        {
            let matches = path.matches("/hello");
            assert_eq!(matches.is_none(), true);
        }
        {
            let matches = path.matches("/hello/hello");
            assert_eq!(matches.is_none(), true);
        }
        {
            let matches = path.matches("/hello/hello/h/e/l/l/o");
            assert_eq!(matches.is_none(), true);
        }
    }

    #[test]
    fn test_parametric_path_with_variable_in_last_position_doesnt_glob_match() {
        let path = Path::new("/files/:path");
        let matches = path.matches("/files/home/user/file.text");
        assert_eq!(matches.is_none(), true);
    }

    #[test]
    fn test_parametric_path_with_variable_in_first_position_doesnt_glob_match() {
        let path = Path::new("/:path");
        let matches = path.matches("/home/user/file.text");
        assert_eq!(matches.is_none(), true);
    }

    #[test]
    fn test_capture_route_parameters() {
        let mut route = "id/friends/:other_id".chars();
        let mut url = "123/friends/555".chars();
        let (key, value) = capture_route_parameter(&mut route, &mut url);
        assert_eq!(key, "id");
        assert_eq!(value, "123");
    }
}

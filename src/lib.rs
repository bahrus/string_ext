pub trait StringExt {
    /// Returns the string before the search string.
    fn substring_before(&self, search: &str) -> String;
    /// Returns the string after the search string.
    fn substring_after(&self, search: &str) -> String;

    /// Returns the string before the last match of the search string.
    fn substring_before_last(&self, search: &str) -> String;

    /// Returns the string between the start and end bookend strings.
    fn substring_between(&self, start: &str, end: &str) -> String;
}

impl StringExt for String {
    fn substring_before(&self , search: &str) -> String{
        let i_pos = self.find(search);
        let answer = match i_pos {
            None => String::from(self),
            Some(val) => self[..val].to_string()
        };
        answer
    }

    fn substring_before_last(&self, search: &str) -> String {
        let i_pos = self.rfind(search);
        let answer = match i_pos {
            None => String::from(self),
            Some(val) => self[..val].to_string()
        };
        answer
    }
    
    fn substring_after(&self, search: &str) -> String{
        let i_pos = self.find(search);
        let answer = match i_pos {
            None => String::new(),
            Some(val) => self[(val + search.len())..].to_string()
        };
        answer
    }

    fn substring_between(&self, start: &str, end: &str) -> String{
        let i_start_pos = self.find(start);
        let answer = match i_start_pos {
            None => String::new(),
            Some(val) => {
                let i_end_pos = self.find(end);
                match i_end_pos {
                    None => String::new(),
                    Some(val2) => self[(val + start.len())..val2].to_string()
                }
            }
        };
        answer
    }
}

#[cfg(test)]
mod tests {
    use crate::StringExt;

    #[test]
    fn test_substring_before() {
        let original = String::from("This is a test of the emergency broadcast system");
        let result = original.substring_before("test");
        assert_eq!(result, "This is a ");
    }

    #[test]
    fn test_substring_before_last(){
        let original = String::from("This is a test of the emergency broadcast test system");
        let result = original.substring_before_last("test");
        assert_eq!(result, "This is a test of the emergency broadcast ");
    }

    #[test]
    fn test_substring_after() {
        let original = String::from("This is a test of the emergency broadcast system");
        let result = original.substring_after("test");
        assert_eq!(result, " of the emergency broadcast system");
    }

    #[test]
    fn test_substring_between(){
        let original = String::from("<html><body>This is a test of the emergency broadcast system</body></html>");
        let result = original.substring_between("<body>", "</body>");
        assert_eq!(result, "This is a test of the emergency broadcast system");
    }
}

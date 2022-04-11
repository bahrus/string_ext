pub trait StringExt {
    fn substring_before(&self, search: &str) -> String;
    fn substring_after(&self, search: &str) -> String;
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
    
    fn substring_after(&self, search: &str) -> String{
        let i_pos = self.find(search);
        let answer = match i_pos {
            None => String::new(),
            Some(val) => self[(val + search.len())..].to_string()
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
    fn test_substring_after() {
        let original = String::from("This is a test of the emergency broadcast system");
        let result = original.substring_after("test");
        assert_eq!(result, " of the emergency broadcast system");
    }
}

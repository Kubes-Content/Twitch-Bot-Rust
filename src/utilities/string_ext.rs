use std::ops::Add;


pub trait AsChars {
    fn as_chars(&self) -> Vec<char>;
}

impl AsChars for str {
    fn as_chars(&self) -> Vec<char> {
        let chars_tmp = self.chars();
        let mut char_vec = Vec::with_capacity(chars_tmp.clone().count());

        for character in chars_tmp { char_vec.push(character); }

        // result
        char_vec
    }
}

pub trait Remove {
    fn remove_within(&self, to_remove:&str) -> String;
}

macro_rules! remove {
    ($self:expr, $to_remove:expr) => {
    {
        let mut remainder_parts = $self.split($to_remove);

        let mut new_value = String::new();

        while let Some(string_part) = remainder_parts.next() {
            new_value = new_value.add(string_part);
        }
        new_value
        };
    }
}

impl Remove for str {
    fn remove_within(&self, to_remove:&str) -> String {
        return remove!(self, to_remove);
    }
}

impl Remove for String {
    fn remove_within(&self, to_remove: &str) -> String {
        return remove!(self, to_remove);
    }
}

pub trait BeginsWith {
    fn begins_with(&self, other:&str) -> bool;
}

impl BeginsWith for str {
    fn begins_with(&self, other: &str) -> bool {
        self.len() >= other.len() && &self[..other.len()] == other
    }
}
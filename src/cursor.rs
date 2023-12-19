pub trait ToCursor<T> {
    fn to_cursor(self, terminator: T) -> Cursor<T>;
}

impl<T: Copy + PartialEq> ToCursor<T> for Vec<T> {
    fn to_cursor(self, terminator: T) -> Cursor<T> {
        Cursor::new(self, terminator)
    }
}

pub struct Cursor<T> {
    buffer: Vec<T>,
    offset: usize,
    end: T,
}

impl<T: Copy + PartialEq> Cursor<T> {
    pub fn new(buffer: Vec<T>, end: T) -> Self {
        Cursor {
            offset: 0usize,
            buffer,
            end,
        }
    }

    pub fn next_or_end(&mut self) -> T {
        if self.is_at_end() {
            self.end
        } else {
            self.offset += 1;
            self.buffer[self.offset - 1]
        }
    }

    pub fn peek(&mut self, count: usize) -> T {
        if self.is_at_end() || self.offset + count >= self.buffer.len() {
            self.end
        } else {
            self.buffer[self.offset + count]
        }
    }

    pub fn peek_first(&mut self) -> T {
        self.peek(0)
    }

    pub fn peek_second(&mut self) -> T {
        self.peek(1)
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn skip_while(&mut self, mut predicate: impl FnMut(T) -> bool) {
        while predicate(self.peek_first()) && !self.is_at_end() {
            self.next_or_end();
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.buffer.len() == self.offset
    }

    pub fn matches(&mut self, item: T) -> bool {
        let next_item = self.peek_first();
        if next_item == item {
            self.next_or_end();
        }

        next_item == item
    }
}

mod tests {

    #[allow(unused_imports)]
    use crate::cursor::ToCursor;

    #[test]
    fn should_peek_items_and_return_next() {
        let mut cursor = "three".chars().collect::<Vec<char>>().to_cursor('\0');

        assert_eq!(cursor.peek_first(), 't');
        assert_eq!(cursor.peek_second(), 'h');
        assert_eq!(cursor.peek(4), 'e');
        assert_eq!(cursor.peek(5), '\0');

        assert_eq!(cursor.next_or_end(), 't');
        assert_eq!(cursor.next_or_end(), 'h');

        assert_eq!(cursor.peek(0), 'r');
        assert_eq!(cursor.peek(1), 'e');

        assert_eq!(cursor.next_or_end(), 'r');
        assert_eq!(cursor.next_or_end(), 'e');
        assert_eq!(cursor.next_or_end(), 'e');

        assert_eq!(cursor.peek(0), '\0');

        assert_eq!(cursor.next_or_end(), '\0');
    }
}

use std::ops::{Add, Sub, Mul, Div};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sub() {
        // let result = add(2, 2);
        let mut my_int: Int8 = Int8::new(6);
        let second_int: Int8 = Int8::new(3);
        my_int = my_int - second_int;
        assert_eq!(my_int, Int8::new(3));
    }
    #[test]
    fn test_add(){
        let mut my_int = Int8::new(6); 
        let second_int = Int8::new(4);
        my_int = my_int + second_int;
        assert_eq!(my_int, Int8::new(10));
    }
    #[test]
    fn test_mult(){
        let mut my_int: Int8 = Int8::new(3);
        let second_int: Int8 = Int8::new(3);
        my_int = my_int * second_int;
        assert_eq!(my_int, Int8::new(9));
    }
    fn empty_fnction(){
        let _my_string = String::from("Hello World!");
    }
}

#[derive(Debug)]
struct Int8{
    _int: Option<i8>
}
impl Int8{
    pub fn new(value: i8) -> Int8{
        let mut _temp = Int8{
            _int: Some(value)
        };
        _temp
    }
}
impl PartialEq for Int8{
    fn eq(&self, other: &Int8) -> bool {
        self._int == other._int
    }
    fn ne(&self, other: &Self) -> bool {
        self._int != other._int
    }
}

// Basic Arithmetic functions for Int8
impl Add for Int8{
    type Output = Int8;
    fn add(self, other: Int8) -> Int8{
        match self._int{
            None => panic!("cannot add nothing!"),
            Some(x) => match other._int{
                None => panic!("cannot add `other` value of None"),
                Some(y) => Self::new(x + y)
            }
        }
    }
}
impl Sub for Int8 {
    type Output = Int8;
    fn sub(self, rhs: Self) -> Self::Output {
        match self._int {
            None => panic!("cannot subtract from None!"),
            Some(x) => match rhs._int {
                None => panic!("cannot subtract with None!"),
                Some(y) => Self::new(x - y)
            }
        }
    }
}
impl Mul for Int8 {
    type Output = Int8;
    fn mul(self, rhs: Self) -> Self::Output {
        match self._int {
            None => panic!("cannot multiply None value"),
            Some(x) => match rhs._int {
                None => panic!("Cannot multiply value `None`"),
                Some(y) => Int8::new(x * y)
            }
        }
    }
}
impl Div for Int8 {
    type Output = Int8;
    fn div(self, rhs: Self) -> Self::Output {
        match self._int {
            None => panic!("Cannot divide None value"),
            Some(x) => match rhs._int {
                None => panic!("Cannot divide with value `None`"),
                Some(y) => Int8::new(x / y)
            }
        }
    }
}
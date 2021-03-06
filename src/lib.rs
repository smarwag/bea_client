#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

mod bea_service;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

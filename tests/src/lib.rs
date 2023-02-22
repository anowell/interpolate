extern crate interpolate;

#[cfg(test)]
mod tests {
    use interpolate::fstring;

    fn f() -> &'static str {
        "hi"
    }

    #[test]
    #[fstring]
    fn no_args() {
        assert_eq!("Hello", f"Hello");
    }

    #[test]
    #[fstring]
    fn interpolation() {
        let name  = "Elsa";
        assert_eq!("Hello, Elsa.", f"Hello, {name}.");
    }

    #[test]
    #[fstring]
    fn ident_collision() {
        let msg  = f();
        let f = "hello";
        assert_eq!("hi", msg);
        assert_eq!("hello", f);
    }

}

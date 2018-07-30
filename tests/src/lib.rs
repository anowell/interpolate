#![feature(use_extern_macros, proc_macro_non_items)]
#![feature(non_ascii_idents)]

extern crate interpolate;

#[cfg(test)]
mod tests {
    use interpolate::s;

    #[test]
    fn wrapped() {
        let name  = "Elsa";
        assert_eq!("Hello, Elsa.", s!("Hello, ${name}."));
        assert_eq!("Hello, Elsa.", s!("Hello, ${ name }."));
        assert_eq!("Hello, Elsa.", s!("Hello, $name."));
    }

    #[test]
    fn prefix() {
        let name  = "Mulan";
        assert_eq!("Mulan, the brave.", s!("${name}, the brave."));
        assert_eq!("Mulan, the brave.", s!("${ name }, the brave."));
        assert_eq!("Mulan, the brave.", s!("$name, the brave."));
    }

    #[test]
    fn suffix() {
        let name  = "Jafar";
        assert_eq!("Vile Jafar", s!("Vile ${name}"));
        assert_eq!("Vile Jafar", s!("Vile ${ name }"));
        assert_eq!("Vile Jafar", s!("Vile $name"));
    }

    #[test]
    fn pre_and_suffix() {
        let name  = "Lilo";
        let name2 = "Stitch";
        assert_eq!("Lilo and Stitch", s!("${name} and ${name2}"));
        assert_eq!("Lilo and Stitch", s!("$name and $name2"));
    }

    #[test]
    fn single_expression_only() {
        let name  = "Prince Charming";
        assert_eq!("Prince Charming", s!("${name}"));
        assert_eq!("Prince Charming", s!("$name"));
    }


    #[test]
    fn connected_expressions() {
        let first_name  = "Mickey";
        let last_name = "Mouse";
        assert_eq!("MickeyMouse", s!("${first_name}${last_name}"));
        assert_eq!("MickeyMouse", s!("$first_name${last_name}"));
        assert_eq!("MickeyMouse", s!("${first_name}$last_name"));
        assert_eq!("MickeyMouse", s!("$first_name$last_name"));
    }

    #[test]
    fn no_expression() {
        assert_eq!("Doc", s!("Doc"));
    }

    #[test]
    fn reuse() {
        let name = "Aladdin";
        assert_eq!(
            "Aladdin is the star of Aladdin.",
            s!("$name is the star of $name.")
        );
    }

    #[test]
    fn xid_start_continue() {
        let _full_name_  = "Judy Hopps";
        assert_eq!("Judy Hopps!", s!("${_full_name_}!"));
        assert_eq!("Judy Hopps!", s!("$_full_name_!"));

        #[allow(non_snake_case)]
        let Ö𞹼𝒜𝟋ↈ  = "Olaf";
        assert_eq!("~Olaf~", s!("~${Ö𞹼𝒜𝟋ↈ}~"));
        assert_eq!("~Olaf~", s!("~$Ö𞹼𝒜𝟋ↈ~"));
    }

    #[test]
    fn expressions() {
        let ducks = vec!["Huey", "Dewey", "Louie"];
        assert_eq!("Dewey", s!("${ducks[1]}"));
        assert_eq!("HueyDeweyLouie", s!("${ducks.concat()}"));

        // Please, don't actually do this
        assert_eq!(
            "hueydeweylouie",
            s!("${ducks.iter().map(|s|s.to_lowercase()).collect::<Vec<_>>().concat()}")
        );
    }
}

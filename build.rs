
use std::io::Write;

include!("src/lib.rs") ;

fn main()
{
    ltr()

}

fn ltr()
{

    let mut file = std::fs::File::create("assets/ltr/mod.rs").unwrap();

    let input_lines = include_str!("assets/ltr/cards_ltr.csv").lines();

    let mut code_lines = vec![] ;
    let mut constants_names = vec![] ;

    code_lines.push("#![allow(non_snake_case)]".to_string());

    for input_line in input_lines
    {
        eprintln!("Observing {}", input_line) ;

        if input_line.trim() == "" { continue ; }

        let var_name =
            {
                let mut iter = input_line.splitn(3, ";") ;
                if iter.next().is_none() // id
                { panic!("Incorrectly formatted csv line") }

                if let Some(n) = iter.next() // name
                {
                    n
                } else {
                    panic!("Malformed csv line")
                }

            }.trim()
            .to_ascii_uppercase()
            .replace(|c| { c == ' ' || c == ',' }, "_")
            .replace("\"", "")
        ;

        code_lines.push(
            format!(
                "pub const {}: &'static str = \"{}\" ;",
                var_name.as_str(),
                input_line.escape_default().collect::<String>().as_str(),
            )
        ) ;

        constants_names.push(var_name) ;

    }

    // Collection
    let mut collection_code = vec!["pub fn collection() -> crate::collections::Collection<'static>\n{\n\tcrate::collections::Collection::make(vec![\n".to_string()] ;

    for constant_name in constants_names {
        collection_code.push(
            [
                "\t\t",
                constant_name.as_str(),
                ",\n"
            ].join("")
        ) ;
    }
    collection_code.push("\n\t])\n}".to_string()) ;

    // Writing out code

    writeln!(file, "{}", code_lines.join("\n")).unwrap() ;
    writeln!(file, "{}", collection_code.join("")).unwrap() ;


    // Save file
    file.flush().unwrap();

}
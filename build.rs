
use std::io::Write;

include!("src/lib.rs") ;

fn main()
{
    ltr()

}

fn ltr()
{

    /*let mut file = std::fs::File::create("assets/ltr/mod.rs").unwrap();

    let lines = include_str!("assets/ltr/index.csv").lines();
    let mut constants = vec![] ;

    writeln!(file, "mod ltr {}", "{").unwrap();
    writeln!(file, "\t#![allow(non_snake_case)]").unwrap();
    writeln!(file, "\tuse crate::CsvConverter ;").unwrap();
    writeln!(file, "\tinclude!(\"header.rs\") ;").unwrap();
    for line in lines
    {
        eprintln!("Observing {}", line) ;
        if line.trim() == "" { continue ; }
        let var: CsvConverter = line.clone().try_into_csv().unwrap() ;

        let var_name = var
            .name()
            .trim()
            .to_ascii_uppercase()
            .replace(|c| { c == ' ' || c == ',' }, "_")
        ;

        writeln!(
            file,
            "\tpub fn {}() -> CsvConverter<'static> {} \"{}\".try_into().unwrap() {}",
            var_name,
            "{",
            line.escape_default(),
            "}"
        ).unwrap();

        constants.push(var_name) ;

    }

    // Collection
    write!(
        file,
        "\tconst COLLECTION: crate::Collection =  {}",
        "crate::Collection(vec!["
    ).unwrap() ;
    for c in constants {
        write!(
            file,
            "(&{}(), 0), ", // todo: bring back header with a type that implements GenericCard
            c
        ).unwrap() ;
    }
    writeln!(file, "{}", "]) }").unwrap();
    writeln!(file, "{}", "}").unwrap();

    file.flush().unwrap();*/

}
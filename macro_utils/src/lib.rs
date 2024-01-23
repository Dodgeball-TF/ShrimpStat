#![feature(proc_macro_quote)]
#![feature(const_option)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]

use proc_macro::TokenStream;

// Custom Macro to generate the EventType enum from every submodule under the events module.
// The field in the enum is PascalCase, the data contained in the enum is the fields name in snake_case and called Inner.
// It should take in the PascalCase variant name and generate the snake_case field name. It should also import the correct events module.

#[proc_macro]
pub fn event_type_enum(_input: TokenStream) -> TokenStream {
    // User provided list of PascalCase event names separated by commas.
    // get event names from token stream
    let input = _input.to_string();
    let event_names: Vec<&str> = input.split(',').collect();

    // A use statement for each event module.
    // use events::server_amend_name;
    let mut use_statements = String::new();
    for event_name in event_names.clone() {
        let event_name = event_name.trim();
        let event_name_snake_case = pascal_to_snake_case(event_name);
        use_statements.push_str(&format!("use crate::models::events::{};\n", event_name_snake_case));
    }

    // enum should look like so
    // enum EventType {
    //     ServerAmendName(server_amend_name::Inner),
    //     ServerRegister(server_register::Inner),
    // }
    let mut event_type_enum = String::from("enum EventType {\n");
    for event_name in event_names {
        let event_name = event_name.trim();
        let event_name_snake_case = pascal_to_snake_case(event_name);
        event_type_enum.push_str(&format!("    {}({}::Inner),\n", event_name, event_name_snake_case));
    }
    event_type_enum.push_str("}\n");

    // return the use statements and the enum
    let output = format!("{}\n{}", use_statements, event_type_enum);
    output.parse().unwrap()
}

fn pascal_to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let mut prev_char = '_';

    for c in s.chars() {
        if c.is_uppercase() {
            if prev_char != '_' && !prev_char.is_uppercase() {
                snake_case.push('_');
            }
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
        prev_char = c;
    }

    snake_case
}
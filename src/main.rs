use std::env;

fn get_literal_match( key: &str ) -> String {
    match env::var( &key ) {
        Ok( value ) => {
            format!(
                "export {n}={v:?}",
                n = &key,
                v = &value
            )
        },
        Err( error ) => {
            format!( "{key}, {error}", key = &key, error = error )
        },
    }
}

fn main() {
    let key = env::args().nth( 1 ).expect( "key_name expected" );
    let matched_key = get_literal_match( &key );
    println!( "{}", matched_key );
}

#[ cfg( test ) ]
mod test {
    use get_literal_match;
    use std::env;

    #[ test ]
    fn variable_exist() {
        let matched = get_literal_match( "HOME" );
        match env::var( "HOME" ) {
            Ok( m ) => {
                let home_match = format!( "export HOME=\"{}\"", m );
                println!( "{}", m );
                assert_eq!( home_match, matched )
            },
            _ => ()
        }
    }

    #[ test ]
    fn variable_doesnt_exist() {
        let key = "NOTAVAR";
        let matched = get_literal_match( &key );
        match env::var( "NOTAVAR" ) {
            Ok( _ ) => (),
            Err( error ) => {
                let not_matched = format!( "{key}, {error}", key = &key, error = error );
                assert_eq!( not_matched, matched );
            }
        }
    }
}

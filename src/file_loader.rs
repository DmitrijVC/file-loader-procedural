#[macro_use] extern crate magic_crypt;

use proc_macro::TokenStream;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use quote::quote;
use magic_crypt::MagicCryptTrait;


#[proc_macro]
pub fn f_load(tokens: TokenStream) -> TokenStream {
    let mut args: Vec<String> = Vec::new();
    args.extend(tokens.into_iter().map(
        |token| {
            token.to_string().replace("\"", "")
        }
    ));

    if args.len() < 2 {
        panic!("f_load macro requires (PATH_TO_FILE, ENCRYPTION_KEY)!");
    }

    let path = Path::new(args.get(0).unwrap());

    if !path.exists() || !path.is_file() {
        panic!("Can't find provided file! [{}] in [{}]", path.display(), std::env::current_dir().unwrap().display());
    }

    let mut buffer = Vec::new();
    match File::open(path) {
        Ok(mut file) => {
            let result = file.read_to_end(&mut buffer);
            if let Err(error) = result {
                panic!("Can't read provided file! [{}]", error.to_string());
            }
        }
        Err(error) => {
            panic!("Can't open provided file! [{}]", error.to_string());
        }
    }

    let mc = new_magic_crypt!(args.get(1).unwrap(), 256);
    let encoded = mc.encrypt_bytes_to_base64(&buffer);

    (quote! {
        #encoded
    }).into()
}

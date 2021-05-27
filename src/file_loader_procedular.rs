#[macro_use] extern crate magic_crypt;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use magic_crypt::MagicCryptTrait;
use random_string::{Charset, generate};

const CHARSET: &'static str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";


/// Macro that reads, and encrypts provided file at compile time
///
/// Returns a result `<KEY>NULL<FILE_CONTENT>`
///
/// # Arguments
///
/// * `file_name`
///
/// # Examples
///
/// ```
/// let encrypted = f_load!("Cargo.toml");
/// ```
#[proc_macro]
pub fn f_load(tokens: TokenStream) -> TokenStream {
    // strenc_initialize!();

    let mut args: Vec<String> = Vec::new();
    for token in tokens.clone() {
        args.push( match token {
            TokenTree::Literal(lit) => lit.to_string().replace("\"", ""),
            _ => tokens.clone().to_string().replace("\"", ""),  // idk
        });
    }

    if args.is_empty() {
        panic!("f_load macro requires PATH_TO_FILE!");
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

    let key = generate(32, &Charset::new(CHARSET).unwrap()).to_string();
    let mc = new_magic_crypt!(&key, 256);
    let encoded = mc.encrypt_bytes_to_base64(&buffer);

    let result = key + "\0" + &*encoded;

    (quote! {
        file_loader::enc!(#result)
    }).into()
}

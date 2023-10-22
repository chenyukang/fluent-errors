#![allow(dead_code)]
#![allow(unused_variables)]
extern crate diag;
use diag::diag;

//#[derive(Diagnostic)]
#[diag(msg = "msg here now", label = "label here")]
struct SignatureRedeclaration {
    pub this: String,
    pub orig: String,
}

fn main() {
    println!("Hello, world!");
    let sig = SignatureRedeclaration {
        this: "foo".into(),
        orig: "bar".into(),
    };

    let (hash, msg) = gen_signature_redeclaration();
    println!("hash: {}", hash);
    println!("msg: {}", msg);

    // dump the Fluent static files

    // translate the Fluent static files
}

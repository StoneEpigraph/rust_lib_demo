use mobi::{Mobi, MobiError};
fn main() -> Result<(), MobiError> {
    let book = vec![0, 0, 0];
    // You can either create a Mobi struct from a slice
    // Or from filesystem
    let m = Mobi::from_path("/home/stone/workspace/rust/lib_demo/static/book/Functional_Interface_In_Java.mobi")?;

    // Access metadata
    let title = m.title();
    let author = m.author().unwrap_or_default();
    let publisher = m.publisher().unwrap_or_default();
    let desc = m.description().unwrap_or_default();
    let isbn = m.isbn().unwrap_or_default();
    let pub_date = m.publish_date().unwrap_or_default();
    let contributor = m.contributor().unwrap_or_default();

    // Access Headers
    let metadata = &m.metadata;
    let header = &metadata.header; // Normal Header
    let pdheader = &metadata.palmdoc; // PalmDOC Header
    let mheader = &metadata.mobi; // MOBI Header
    let exth = &metadata.exth; // Extra Header

    // Access content
    let content = m.content_as_string();

    println!("title: {}", title);
    println!("author: {}", author);
    println!("publisher: {}", publisher);
    println!("description: {}", desc);
    println!("isbn: {}", isbn);
    println!("pub_date: {}", pub_date);
    println!("contributor: {}", contributor);
    println!("metadata: {:?}", metadata);
    println!("header: {:?}", header);
    println!("pdheader: {:?}", pdheader);
    println!("mheader: {:?}", mheader);
    println!("exth: {:?}", exth);

    Ok(())
}
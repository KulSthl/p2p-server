
/// simple handler
async fn index(req: HttpRequest, session: Session) -> Result<impl Responder, Error> {
    println!("{:?}", req);

    // session
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }

    Ok("Welcome!")
}

use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let uuid = uuid::Uuid::new_v4();
    let uuid = uuid.to_string();
    let (client, conn) = tokio_postgres::connect("host=localhost user=postgres password=prisma", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute("DROP TABLE IF EXISTS withuuid", &[]).await?;
    client.execute("CREATE TABLE IF NOT EXISTS withuuid (u uuid, n int)", &[]).await?;

    dbg!("got here");

    let stmt = client.prepare
        ("INSERT INTO \"withuuid\" (\"u\", \"n\") VALUES ($1, $2)")
        .await?;

    dbg!("prepared");

    let rows = client.execute(&stmt, &[&uuid, &3i32]).await?;

    // let rows = client
    //     .query("INSERT INTO \"withuuid\" (\"u\", \"n\") SELECT text($1), $2", &[&uuid, &3i32])
    //     .await?;

    dbg!(rows);
    Ok(())
}

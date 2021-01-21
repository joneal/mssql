use odbc_iter::{Odbc};

fn main() {
    const CONN_STRING: &str = "Driver={SQL Server};Server=devsql.samtec.ad;Database=production;Uid=AwsSgn;Pwd=94B1CC55E12192FF4CBF0129B7E7DC0541F556EE$;";
    // Connect to database using connection string

    let mut connection = Odbc::connect(&CONN_STRING).expect("failed to connect to DEVSQL");

    // Handle statically guards access to connection and provides query functionality
    let mut db = connection.handle();

    for row in db.query::<(i16, String, String, String)>(
        "SELECT TOP 100 USER_MASTER_ID, USER_NAME, FIRST_NAME, LAST_NAME
         FROM dbo.USER_MASTER WHERE USER_MASTER_ID > 6")
        .expect("failed to run query") {
        let (id, username, first_name, last_name) = row.expect("failed to fetch row");
        println!("{} {} {} {}", id, username, first_name, last_name);
    }
}

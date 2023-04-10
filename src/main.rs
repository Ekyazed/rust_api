use lib::{self, models::users::{update_user, NewUser}};

fn main() {
    let updating_user = NewUser {
        username: "Dorian_updated",
        email: "dorian.updated@cgi.com",
        password: "fr_updated",
    };

   match update_user(2, &updating_user) {
    Ok(row_updated) => {
        if row_updated == 0 {
            println!("Aucune ligne modifier")
        }
    },
    Err(e) => println!("{:?}", e)
   }
}

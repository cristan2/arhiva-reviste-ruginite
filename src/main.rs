// use rusqlite::Result;
use arhiva_reviste_ruginite::db::*;
use arhiva_reviste_ruginite::printer::{PrintableRow, make_table};

fn main() /*-> Result<()>*/ {

    let db = DBConnection::open("arhiva_reviste_v7.0.db");

    afisare_toate_revistele(&db);
    afisare_toate_editiile(&db);

    // citire si afisare revista dupa id
    // let rev_id = 1;
    // let revista: Result<Revista> = db.retrieve_revista(&rev_id);
    //
    // println!("{:?}", revista);

    /*Ok(())*/
}

fn afisare_toate_revistele(db: &DBConnection) {
    let reviste = db.retrieve_toate_revistele();
    let tabel_reviste = make_table(&reviste);
    tabel_reviste.printstd();
}

fn afisare_toate_editiile(db: &DBConnection) {
    let editii = db.retrieve_toate_editiile();
    let tabel_editii = make_table(&editii);
    tabel_editii.printstd();
}
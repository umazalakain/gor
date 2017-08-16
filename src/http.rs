use std::io;
use std::sync::Mutex;
use std::path::{Path, PathBuf};

use rocket;
use rocket::State;
use rocket::response::NamedFile;
use rocket_contrib::Json;

use super::board::*;

/*
 * Ideas for a simple game server:
 *
 * Server state:
 * - Client contacts server
 * - Server responds with list of ongoing games and list of game requests.
 * - Client can view a game
 * - Client can register
 *   - Provides nickname
 *   - Receives token
 * - Registered client can accept game request
 * - Registered client can create game request
 * - Registered client can list own ongoing games
 *
 * Game state:
 * - Board matrix
 * - Last move
 * - Current player
 * - Capture count
 *
 * Cliend sends move signed with token
 */

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("src/static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("src/static/").join(file)).ok()
}

#[get("/", format = "application/json")]
fn get_game(state: State<Mutex<Game>>) -> Json<Game> {
    let game = state.lock().unwrap();
    Json(game.clone())
}

#[post("/", format = "application/json", data = "<req>")]
fn make_move(state: State<Mutex<Game>>, req: Json<Move>) -> Json<Result<Game,IllegalMove>> {
    let Json(m) = req;
    let mut game = state.lock().unwrap();
    Json(match game.make_move(m) {
        Ok(_) => Ok(game.clone()),
        Err(err) => Err(err),
    })
}

pub fn serve() {
    let game = Mutex::new(Game::new());
    rocket::ignite()
        .manage(game)
        .mount("/api", routes![get_game, make_move])
        .mount("/", routes![index, files])
        .launch();
}

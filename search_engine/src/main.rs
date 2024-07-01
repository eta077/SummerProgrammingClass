use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    println!("args: {args:?}");
    let search_string = args.join(" ");
    println!("search string: {search_string:?}");

    let mut api = String::from("https://pokeapi.co/api/v2/pokemon/");
    api = api + &search_string;

    let body = reqwest::get(api).await?.json::<ResultBody>().await?;

    let result = body
        .game_indices
        .into_iter()
        .map(|game_index| game_index.version)
        .map(|version| version.name)
        .collect::<Vec<String>>();

    println!("result = {result:?}");

    Ok(())
}

#[derive(Debug, Deserialize)]
struct ResultBody {
    game_indices: Vec<GameEntry>,
}

#[derive(Debug, Deserialize)]
struct GameEntry {
    version: GameVersion,
}

#[derive(Debug, Deserialize)]
struct GameVersion {
    name: String,
}

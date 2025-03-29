use std::collections::HashSet;

use tokio::fs::read_to_string;

pub async fn init_mods() -> HashSet<String> {
    let mut temp = HashSet::new();

    for line in read_to_string("./mods.csv").await.unwrap().split("\n") {
        let name = line
            .replace("\t", "")
            .replace(" ", "")
            .replace("\n", "")
            .to_lowercase();
        temp.insert(name);
    }
    temp
}

pub async fn load_channel() -> String {
    read_to_string("./channel")
        .await
        .unwrap()
        .replace("\n", "")
        .replace(" ", "")
}

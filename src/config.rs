use std::collections::HashSet;

pub async fn init_mods() -> HashSet<String> {
    let mut temp = HashSet::new();

    for line in read_or_create_default("mods", "heroaaxtwitchtrollwieder\n")
        .await
        .split("\n")
    {
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
    read_or_create_default("channel", "misterjp1987\n")
        .await
        .replace("\n", "")
        .replace(" ", "")
}

async fn read_or_create_default(path: &str, default: &str) -> String {
    let path_struc = std::path::Path::new(path);
    if path_struc.exists() {
        tokio::fs::read_to_string(path_struc).await.unwrap()
    } else {
        tokio::fs::write(path, default).await.unwrap();
        default.to_owned()
    }
}

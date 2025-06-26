use soroban_sdk::{contracttype, symbol_short, Env, String, Symbol, Vec};

const CONTENT_KEY: Symbol = symbol_short!("CONTENT");
const NEXT_ID_KEY: Symbol = symbol_short!("NEXT_ID");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Content {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub subject_tags: Vec<String>,
    pub content_url: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentList {
    pub contents: Vec<Content>,
}

pub struct ContentStorage;

impl ContentStorage {
    pub fn set_content(env: &Env, content: &Content) {
        let storage = env.storage().instance();
        let mut content_list = if storage.has(&CONTENT_KEY) {
            storage.get(&CONTENT_KEY).unwrap()
        } else {
            ContentList {
                contents: Vec::new(env),
            }
        };

        // Actualizar o añadir el contenido
        let mut found = false;
        for i in 0..content_list.contents.len() {
            if content_list.contents.get_unchecked(i).id == content.id {
                content_list.contents.set(i, content.clone());
                found = true;
                break;
            }
        }

        if !found {
            content_list.contents.push_back(content.clone());
        }

        storage.set(&CONTENT_KEY, &content_list);
        storage.extend_ttl(50, 100);
    }

    pub fn get_all_content(env: &Env) -> Vec<Content> {
        let storage = env.storage().instance();
        if !storage.has(&CONTENT_KEY) {
            return Vec::new(env);
        }

        let content_list: ContentList = storage.get(&CONTENT_KEY).unwrap();
        content_list.contents
    }

    pub fn initialize(env: &Env) {
        let storage = env.storage().instance();

        // Inicializar la lista de contenido si no existe
        if !storage.has(&CONTENT_KEY) {
            let content_list = ContentList {
                contents: Vec::new(env),
            };
            storage.set(&CONTENT_KEY, &content_list);
        }

        // Inicializar el contador de ID si no existe
        if !storage.has(&NEXT_ID_KEY) {
            storage.set(&NEXT_ID_KEY, &0u64);
        }

        // Extender el TTL del almacenamiento
        storage.extend_ttl(50, 100);
    }
}

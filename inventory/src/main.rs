use tracing::{error, warn};

fn main() {
    tracing_subscriber::fmt::init();
    let query = parse_query_from_args();
    println!("{query:?}");

    let items = vec![
        Item {
            id: String::from("asdf"),
            quantity: 1,
            size: ItemSize::Small,
        },
        Item {
            id: String::from("123"),
            quantity: 1,
            size: ItemSize::Small,
        },
    ];

    let mut matching_items = Vec::new();

    for item in items {
        if item.matches(&query) {
            matching_items.push(item);
        }
    }

    println!("{matching_items:#?}");
}

fn parse_query_from_args() -> ItemQuery {
    let mut query = ItemQuery::default();
    for arg in std::env::args().skip(1) {
        let mut parts = arg.split('=');
        let key = parts.next().unwrap();
        if let Some(value) = parts.next() {
            match key {
                "id" => query.id = Some(value.to_owned()),
                "quantity" => match value.parse() {
                    Ok(quantity) => query.quantity = Some(quantity),
                    Err(e) => error!(error = ?e, "Unable to parse quantity value"),
                },
                "size" => match ItemSize::try_from(value) {
                    Ok(size) => query.size = Some(size),
                    Err(e) => error!(error = ?e, "Unable to parse size value"),
                },
                other => warn!("Unknown query parameter: {other}"),
            }
        }
    }

    query
}

#[derive(Debug)]
struct Item {
    id: String,
    quantity: u16,
    size: ItemSize,
}

impl Item {
    pub fn matches(&self, query: &ItemQuery) -> bool {
        if let Some(query_id) = &query.id {
            if &self.id == query_id {
                return true;
            }
        }
        if let Some(query_quantity) = &query.quantity {
            if &self.quantity == query_quantity {
                return true;
            }
        }
        if let Some(query_size) = &query.size {
            if &self.size == query_size {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ItemSize {
    Small,
    Medium,
    Large,
}

impl TryFrom<&str> for ItemSize {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "small" => Ok(ItemSize::Small),
            "medium" => Ok(ItemSize::Medium),
            "large" => Ok(ItemSize::Large),
            _ => Err(String::from("Unknown size value")),
        }
    }
}

#[derive(Debug, Default)]
struct ItemQuery {
    id: Option<String>,
    quantity: Option<u16>,
    size: Option<ItemSize>,
}

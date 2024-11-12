use redux_rs::{Selector, Store};

#[derive(Default, Debug)]
enum Stage {
    #[default]
    Collect,
    Group,
    Vote,
    Discuss,
}

#[derive(Debug, Clone)]
struct Topic {
    text: String,
    author: String,
}

#[derive(Debug, Clone)]
struct Category {
    name: String,
    order: u32,
    topics: Vec<Topic>,
}

#[derive(Default, Debug)]
struct State {
    categories: Vec<Category>,
    stage: Stage,
}

#[derive(Debug)]
enum Action {
    AddCategory { category: Category },
    AddTopic { category: String, topic: Topic },
}

fn reducer(mut state: State, action: Action) -> State {
    match action {
        Action::AddCategory { category } => State {
            categories: {
                state.categories.push(category);
                state.categories
            },
            ..state
        },
        Action::AddTopic { category, topic } => State {
            categories: state
                .categories
                .into_iter()
                .map(|mut x| {
                    if x.name == category {
                        let mut topics = x.topics.clone();
                        topics.push(topic.clone());
                        x.topics = topics
                    }

                    x
                })
                .collect(),
            ..state
        },
    }
}

struct SelectCategories;
impl Selector<State> for SelectCategories {
    type Result = Vec<Category>;

    fn select(&self, state: &State) -> Self::Result {
        state.categories.clone()
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new(reducer);

    store
        .dispatch(Action::AddCategory {
            category: Category {
                name: String::from("Category 1"),
                order: 1,
                topics: vec![],
            },
        })
        .await;

    store
        .dispatch(Action::AddCategory {
            category: Category {
                name: String::from("Category 2"),
                order: 2,
                topics: vec![],
            },
        })
        .await;

    store
        .dispatch(Action::AddTopic {
            category: String::from("Category 1"),
            topic: Topic {
                text: String::from("Test Topic for category 1"),
                author: String::from("Nate"),
            },
        })
        .await;

    store
        .dispatch(Action::AddTopic {
            category: String::from("Category 1"),
            topic: Topic {
                text: String::from("Another test Topic for category 1"),
                author: String::from("Nate"),
            },
        })
        .await;

    store
        .dispatch(Action::AddTopic {
            category: String::from("Category 2"),
            topic: Topic {
                text: String::from("Test Topic for category 2"),
                author: String::from("Nate"),
            },
        })
        .await;

    println!("{:?}", store.select(SelectCategories).await);
}

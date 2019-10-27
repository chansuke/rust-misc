pub trait Aggregate {
    fn aggregate(&self) -> String;
}

pub struct Instagram {
    pub title: String,
    pub description String,
    pub photo_url String,
    pub author String,
}

impl Aggregate for Instagram {
    fn aggregate(&self) -> String {
        format!("This {} is taken by {}", self.photo_url, self.author)
    }
}

//pub fn notification(item: impl Aggregate) {
//    println!("news {}", item.aggregate());
//}

pub fn notifucation<T: Summary>(item: T) {
    println!("news {}", item.aggregate());
}

pub struct Twitter {
    pub username: String,
    pub tweet: String,
    pub retweet: bool,
}

impl Aggregate for Twitter {
    fn aggregate(&self) -> String {
        format!("{} {} {}", self.username, self.tweet, self.author)
    }
}

let tweet = Twitter {
    username: String::from("michael"),
    tweet: String::from("This is my first tweet"),
    retweet: true,
};

println!("{}", tweet.aggregate());


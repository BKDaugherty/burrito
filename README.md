# Burrito Tracker
My friends and I at UCLA really like burritos. We created a groupme where everytime we eat a burrito, we send a message in the groupchat, and increment a counter. I wanted to get some base level analytics going on the chat, and also wanted a quick experiment to do with the Rust Programming language. The result was a quick script (that took me a while at the time) that got some of the information I wanted. Sadly due to some members of the rowing team pulling some shit, we had to delete the original groupme conversation, so our stats look sad. Either way, I hope to at some point run some more complicated queries on this data!

## Usage

First create a .env file like this

```
GROUPME_API_TOKEN="You're api token goes here"
GROUPME_CONVERSATION_ID="You're conversation id"
```

Then run
```
cargo run -- --notable-burrito-count 60
```

Get this!

```
brendond@brendond-mbp burrito % cargo run -- --notable-burrito-count 60
   Compiling burrito v0.1.0 (/Users/brendond/Dev/burrito)
    Finished dev [unoptimized + debuginfo] target(s) in 3.17s
     Running `target/debug/burrito --notable-burrito-count 60`
UserData {
    name: "Shon Mackie",
    message_count: 70,
    message_with_image_count: 33,
    number_of_likes: 185,
}
UserData {
    name: "Adam Key",
    message_count: 83,
    message_with_image_count: 58,
    number_of_likes: 292,
}
UserData {
    name: "Jackson Boulter",
    message_count: 81,
    message_with_image_count: 38,
    number_of_likes: 302,
}
UserData {
    name: "Lacey Cappos",
    message_count: 70,
    message_with_image_count: 32,
    number_of_likes: 238,
}
UserData {
    name: "Brendon Daugherty",
    message_count: 137,
    message_with_image_count: 72,
    number_of_likes: 328,
}
UserData {
    name: "Evie Malamut",
    message_count: 66,
    message_with_image_count: 34,
    number_of_likes: 169,
}
UserData {
    name: "Simon Conover",
    message_count: 66,
    message_with_image_count: 49,
    number_of_likes: 320,
}
UserData {
    name: "Hannah Brenchley",
    message_count: 64,
    message_with_image_count: 22,
    number_of_likes: 201,
}
UserData {
    name: "Mandy Hagen",
    message_count: 87,
    message_with_image_count: 53,
    number_of_likes: 338,
}
UserData {
    name: "Kellen McCabe",
    message_count: 78,
    message_with_image_count: 32,
    number_of_likes: 227,
}
UserData {
    name: "Matt Rubly",
    message_count: 93,
    message_with_image_count: 80,
    number_of_likes: 351,
}
UserData {
    name: "John Steenstra",
    message_count: 78,
    message_with_image_count: 52,
    number_of_likes: 145,
}
```
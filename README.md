# SIMPLETGS, the unneeded telegram crate for Rust

Simpletgs is a extremely simple library to send photos to a telegram group. It lacks a lot of features just because at the time of writing jeltelbot, a bot writted to check the RSS of [jugandoenlinux.com](https://www.jugandoenlinux.com) and send the new posts to our telegram [group](https://t.me/jugandoenlinux), the crate [telegram-bot](https://github.com/telegram-rs/telegram-bot) didn't support the functionality sendPhoto from telegram API which is the type of message we want, otherwise, this crate won't exist, as that crate is much more featured and it works very well.

# EXAMPLE (a sort of)
As this library doesn't do too much, if you are looking for a simple way to send photos to groups and chats, you can use it as follows:

``` rust
use simpletgrs;

static TELEGRAM_TOKEN: &str = "YOUR_TELEGRAM_TOKEN";
static TELEGRAM_CHAT: &str = "CHAT/GROUP_TELEGRAM_ID";

let message = "<b>Hello channel!</b";
let photo = "/path/to/picture";

let sended = simpletgrs::send_photo(photo, message, TELEGRAM_TOKEN, TELEGRAM_CHAT);
if sended.is_ok() == true {
    let _result = simpletgrs::pin_chat_message(sended.unwrap(), TELEGRAM_TOKEN);
}

```

# FUTURE, MAINTENANCE AND SUPPORT
As this library exists because [telegram-bot](https://github.com/telegram-rs/telegram-bot) didn't support sendPhoto, as soon as it does, I will stop using this crate, so don't expect newer features. I only will release new versions if there is a critical bug/issue.
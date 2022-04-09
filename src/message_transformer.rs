pub fn make_message_patriotic(mut message: String) -> String {
    message = message.replace('з', "z");
    message = message.replace('З', "Z");
    message = message.replace('в', "v");
    message = message.replace('В', "V");
    message
}

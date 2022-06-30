fn main() {
    println!("Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song");

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eight",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let lyrics = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swanms a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for index in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", days[index]);
        for i in (0..index + 1).rev() {
            if i == 0 && index != 0 {
                println!("And {}", lyrics[i]);
            } else {
                println!("{}", lyrics[i]);
            }
        }
        println!("");
    }
}

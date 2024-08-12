fn main()
{
    print!("{} days\n", 31);

    print!("{0}, {1}, {1}, {2}\n", "A", "B", "C");

    print!("Room 1 = {room1}, Room 2 = {room2}, Room 3 = {room3}\n", room1="Doraemon", room2="Pokemon", room3="Dizimon");

    print!("12 in bin {:b}\n", 12);
    print!("{:>5}\n",5);
    print!("{number:0>5}\n", number=13);
    print!("{number:0<5}\n", number=13);

    print!("{number:0>width$}", number=12, width=15);


    #[allow(dead_code)]
    struct Structure(i32);

    

}
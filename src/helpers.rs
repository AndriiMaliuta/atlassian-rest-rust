pub mod helpers {
    use rand::Rng;

    pub fn rand_string(size: i32) -> String {
        let text = "lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
            labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip \
            ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu \
            fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui \
            officia deserunt mollit anim id est laborum.";
        let words = text.split(" ").collect::<Vec<&str>>();
        let mut lorem = "".to_string();
        for a in 0..size {
            let mut num = rand::thread_rng().gen_range(0..words.len());
            lorem = [lorem, words[num].to_string()].join(" ");
        }
        lorem.to_string()
    }
}
use rand::Rng;

pub fn encrypt(x:i32) -> String {
    let wordlist = ['a','b','c','d','e','f','g','h','i','j','k','l','m',
        'n','o','p','q','r','s','t','u','v','w','x','y','z','1','2','3','4','5','6','7'];
    let mut key = String::new();
    for it in 0..x {
        let randtonumber :usize = rand::thread_rng().gen_range(0..wordlist.len());
        key.push(wordlist[randtonumber])
        
    };
    return key
}

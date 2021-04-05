fn bitwise_addition_mod_3(nums: &[usize]) {
    let mut one = 0;
    let mut one_c = 0;
    let mut two = 0;
    let mut two_c = 0;
    let mut three = 0;
    for i in 0..nums.len() {
        println!("{:#032b}", nums[i]);
        one_c = (one & nums[i]);
        one = (one ^ nums[i]);
        two_c = (two & one_c);
        two = (two ^ one_c);
        three = (three ^ two_c);
    }
    println!("--------");
    println!("{:#032b}", one);
    println!("{:#032b}", two);
}

fn bitwise_addition_mod_3a(nums: &[usize]) {    
    let mut one = 0;
    let mut two = 0;
    for i in 0..nums.len() {
        println!("{:#032b}", nums[i]);
        one = !two & (one ^ nums[i]);
        two = !one & (two ^ nums[i]);
    }
    println!("--------");
    println!("{:#032b}", one);
    println!("{:#032b}", two);
}

fn bitwise_addition_mod_4(nums: &[usize]) {    
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    for i in 0..nums.len() {
        println!("{:#032b}", nums[i]);
        one = (!two & !three) & (one ^ nums[i]);
        two = (!one & !three) & (two ^ nums[i]);
        three = (!one & !two) & (three ^ nums[i]);
    }
    println!("--------");
    println!("{:#032b}", one);
    println!("{:#032b}", two);
    println!("{:#032b}", three);
}

fn main() {
    let nums = [11,21,3,40,17,2,8,7];    
    // println!("BITWISE MODULO THREE OLD");
    // bitwise_addition_mod_3a(&nums);

    // println!("BITWISE MODULO THREE");
    // bitwise_addition_mod_3(&nums);

    println!("BITWISE MODULO FOUR");
    bitwise_addition_mod_4(&nums);
}
fn main() {
    //variables();
    integer();
}


//Method to show variables
fn variables()
{
    let company_string = "TutorialsPoint";  // string type
    let rating_float = 4.5;                 // float type
    let is_growing_boolean = true;          // boolean type
    let icon_char = '♥';                    //unicode character type
 
    /*
        The println! macro takes two arguments −

        A special syntax { }, which is the placeholder
        The variable name or a constant
        The placeholder will be replaced by the variable’s value
    */
    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("company icon is:{}",icon_char);
}

fn integer()
{
    let result = 10;    // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;
    println!("result value is {}",result);
    println!("sum is {} and age is {}",sum,age);
    println!("mark is {} and count is {}",mark,count);
}
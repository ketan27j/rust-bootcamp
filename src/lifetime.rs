fn main(){
    let str1 = String::from("Ketan");
    let str2 = String::from("Juikar");
    let ans = longest_string(&str1, &str2);
    print!("{:?}",ans);
}
fn longest_string<'a>(s1: &'a String, s2: &'a String)-> &'a String{
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}
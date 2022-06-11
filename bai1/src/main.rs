fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let mut i = 0;
    let mut j = 0;
    while i < org_arr.len() && j < sub_arr.len() {
        println!("go {} {}", i, j);
        if org_arr[i] == sub_arr[j] {
            i = i + 1;
            j = j + 1;
            if j == sub_arr.len() {
                println!("YES");
                return;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    println!("No");
}

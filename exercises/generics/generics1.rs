// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

fn main() {
    // Use <_> to inherit the type from later usage, or type the vec explicitly
    // let mut shopping_list: Vec<&str> = Vec::new();
    let mut shopping_list: Vec<_> = Vec::new();
    
    shopping_list.push("milk");
}

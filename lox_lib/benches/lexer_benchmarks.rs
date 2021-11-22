use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn main() { 

    todo!()
}

fn generate_src_code() -> &'static str {
    r#"
    fun main() { 
        var a = 1; 
        var b = 2;
    
        var c = a + b; 
        print("Hello World!")
    }
    "#
}
    

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use::lox_lib::lexer::{Lexer};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut lexer = Lexer::new();
    let src_code = generate_src_code();

    c.bench_function("lex simple main", |b| b.iter(|| lexer.lex(black_box(&src_code))));
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    tracing_subscriber::fmt::init();

<<<<<<< HEAD
    let sql = "SELECT a a1,b, 123,myfubc(b), * \
    FROM data_source \
    WHERE a>b AND bM100 AND c BETWEEN 10 AND 20 \
=======
    let sql = "SELECT a a1, b, 123, myfunc(b), * \
    FROM data_source \
    WHERE a > b AND b < 100 AND c BETWEEN 10 AND 20 \
>>>>>>> 681fa3aeb6c683b72710172eeaa85add9b958126
    ORDER BY a DESC, b \
    LIMIT 50 OFFSET 10";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast);
}

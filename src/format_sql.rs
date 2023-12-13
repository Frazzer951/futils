use anyhow::Result;
use sqlparse::{FormatOption, Formatter};

pub fn format_sql_query(sql_query: String) -> Result<String> {
    let mut f = Formatter::default();
    let mut options = FormatOption::default();
    options.reindent = true;
    options.reindent_aligned = true;
    options.strip_whitespace = true;
    options.keyword_case = "lower";
    options.use_space_around_operators = true;

    let formatted_sql = f.format(&sql_query, &mut options);

    Ok(formatted_sql)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_sql_query_simple() {
        let sql_query = "SELECT * FROM foo".to_string();
        let formatted_sql = format_sql_query(sql_query).unwrap();
        let expected = "\
select *
  from foo";
        assert_eq!(formatted_sql, expected);
    }

    #[test]
    fn test_format_sql_query() {
        let sql_query = "select * from table where a =0 and b<>4 group by id limit 10;".to_string();
        let formatted_sql = format_sql_query(sql_query).unwrap();
        let expected = "\
select *
  from table
 where a = 0
   and b <> 4
 group by id
 limit 10;";
        assert_eq!(formatted_sql, expected);
    }
}

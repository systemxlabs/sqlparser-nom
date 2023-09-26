use sqlparser_nom::parse_query;

#[test]
pub fn test_query() {
    let cases = [
        // with
        (
            r#"WITH x AS (SELECT a, MAX(b) AS b FROM t GROUP BY a) SELECT a, b FROM x;"#,
            r#"WITH x AS (SELECT a, MAX(b) AS b FROM t GROUP BY a) SELECT a, b FROM x"#,
        ),
        // select
        (
            r#"SELECT a, b, a + b FROM table"#,
            r#"SELECT a, b, (a + b) FROM table"#,
        ),
        (
            r#"SELECT DISTINCT person, age FROM employees"#,
            r#"SELECT DISTINCT person, age FROM employees"#,
        ),
        // from
        (
            r#"SELECT t.a FROM table AS t"#,
            r#"SELECT t.a FROM table AS t"#,
        ),
        // where
        (
            r#"SELECT a FROM table WHERE a > 10"#,
            r#"SELECT a FROM table WHERE (a > 10)"#,
        ),
        // join
        (
            r#"select * from x inner join x y ON x.column_1 = y.column_1;"#,
            r#"SELECT * FROM (x INNER JOIN x AS y ON (x.column_1 = y.column_1))"#,
        ),
        (
            r#"select * from x left join x y ON x.column_1 = y.column_2;"#,
            r#"SELECT * FROM (x LEFT OUTER JOIN x AS y ON (x.column_1 = y.column_2))"#,
        ),
        (
            r#"select * from x right join x y ON x.column_1 = y.column_2;"#,
            r#"SELECT * FROM (x RIGHT OUTER JOIN x AS y ON (x.column_1 = y.column_2))"#,
        ),
        (
            r#"select * from x full outer join x y ON x.column_1 = y.column_2;"#,
            r#"SELECT * FROM (x FULL OUTER JOIN x AS y ON (x.column_1 = y.column_2))"#,
        ),
        // (r#"select * from x natural join x y;"#, r#""#),
        // (r#"select * from x cross join x y;"#, r#""#),
        // group by
        (
            r#"SELECT a, b, MAX(c) FROM table GROUP BY a, b"#,
            r#"SELECT a, b, MAX(c) FROM table GROUP BY a, b"#,
        ),
        // (r#"SELECT a, b, ARRAY_AGG(c, ORDER BY d) FROM table GROUP BY a, b"#, r#""#),
        // having
        (
            r#"SELECT a, b, MAX(c) FROM table GROUP BY a, b HAVING MAX(c) > 10"#,
            r#"SELECT a, b, MAX(c) FROM table GROUP BY a, b Having (MAX(c) > 10)"#,
        ),
        // order by
        (
            r#"SELECT age, person FROM table ORDER BY age;"#,
            r#"SELECT age, person FROM table ORDER BY age"#,
        ),
        (
            r#"SELECT age, person FROM table ORDER BY age DESC;"#,
            r#"SELECT age, person FROM table ORDER BY age DESC"#,
        ),
        (
            r#"SELECT age, person FROM table ORDER BY age, person DESC;"#,
            r#"SELECT age, person FROM table ORDER BY age, person DESC"#,
        ),
        // limit
        (
            r#"SELECT age, person FROM table LIMIT 10"#,
            r#"SELECT age, person FROM table LIMIT 10"#,
        ),
        // except/exclude
        (
            r#"SELECT * EXCEPT(age, person) FROM table;"#,
            r#"SELECT * EXCEPT (age, person) FROM table"#,
        ),
        (
            r#"SELECT * EXCLUDE(age, person) FROM table;"#,
            r#"SELECT * EXCLUDE (age, person) FROM table"#,
        ),
        // subquery
        (
            r#"select * from x y where exists (select * from x where x.column_1 = y.column_1);"#,
            r#"SELECT * FROM x AS y WHERE EXISTS (SELECT * FROM x WHERE (x.column_1 = y.column_1))"#,
        ),
        (
            r#"select * from x y where not exists (select * from x where x.column_1 = y.column_1);"#,
            r#"SELECT * FROM x AS y WHERE NOT EXISTS (SELECT * FROM x WHERE (x.column_1 = y.column_1))"#,
        ),
        (
            r#"select * from x where column_1 in (select column_1 from x);"#,
            r#"SELECT * FROM x WHERE column_1 IN (SELECT column_1 FROM x)"#,
        ),
        (
            r#"select * from x where column_1 not in (select column_1 from x);"#,
            r#"SELECT * FROM x WHERE column_1 NOT IN (SELECT column_1 FROM x)"#,
        ),
        (
            r#"select * from x y where column_1 < (select sum(column_2) from x where x.column_1 = y.column_1);"#,
            r#"SELECT * FROM x AS y WHERE (column_1 < (SELECT sum(column_2) FROM x WHERE (x.column_1 = y.column_1)))"#,
        ),
        // window function
        (
            r#"SELECT depname, empno, salary, avg(salary) OVER (PARTITION BY depname) FROM empsalary;"#,
            r#"SELECT depname, empno, salary, avg(salary) OVER (PARTITION BY depname) FROM empsalary"#,
        ),
        (
            r#"SELECT depname, empno, salary, rank() OVER (PARTITION BY depname ORDER BY salary DESC) FROM empsalary;"#,
            r#"SELECT depname, empno, salary, rank() OVER (PARTITION BY depname ORDER BY salary DESC) FROM empsalary"#,
        ),
        // (r#"SELECT depname, empno, salary, avg(salary) OVER(ORDER BY salary ASC ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING) AS avg, min(salary) OVER(ORDER BY empno ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS cum_min FROM empsalary ORDER BY empno ASC;"#, r#""#),
        (
            r#"SELECT sum(salary) OVER w, avg(salary) OVER w FROM empsalary WINDOW w AS (PARTITION BY depname ORDER BY salary DESC);"#,
            r#"SELECT sum(salary) OVER w, avg(salary) OVER w FROM empsalary WINDOW w AS (PARTITION BY depname ORDER BY salary DESC)"#,
        ),
    ];
    for (input, output) in &cases {
        let result = parse_query(input);
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(output, &result.to_string());
    }
}

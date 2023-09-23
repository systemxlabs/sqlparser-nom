# sqlparser-nom 
A experimental SQL query parser using nom.

- [ ] Query
  - [x] Select
  - [ ] From
  - [x] Where
  - [x] Order by
  - [x] Limit
  - [ ] CTE
  - [ ] Into
  - [x] Group by
  - [x] Having
  - [x] Aggregate
  - [ ] Window
- [x] Pratt Parsing
- [ ] Friendly error info

## Example
```sql
select a, t.b, c 
from t 
where a > ((1 + 2) * 3) and b < c 
group by a, c 
order by a, b desc 
limit 1, 2
```
output ast:
```
SelectStatement {
    body: Select {
        projection: [
            UnnamedExpr(
                ColumnRef {
                    database: None,
                    table: None,
                    column: Ident {
                        value: "a",
                    },
                },
            ),
            UnnamedExpr(
                ColumnRef {
                    database: None,
                    table: Some(
                        Ident {
                            value: "t",
                        },
                    ),
                    column: Ident {
                        value: "b",
                    },
                },
            ),
            UnnamedExpr(
                ColumnRef {
                    database: None,
                    table: None,
                    column: Ident {
                        value: "c",
                    },
                },
            ),
        ],
        from: Ident {
            value: "t",
        },
        selection: Some(
            BinaryOp {
                left: BinaryOp {
                    left: ColumnRef {
                        database: None,
                        table: None,
                        column: Ident {
                            value: "a",
                        },
                    },
                    op: Gt,
                    right: BinaryOp {
                        left: BinaryOp {
                            left: Literal(
                                UnsignedInteger(
                                    1,
                                ),
                            ),
                            op: Add,
                            right: Literal(
                                UnsignedInteger(
                                    2,
                                ),
                            ),
                        },
                        op: Mul,
                        right: Literal(
                            UnsignedInteger(
                                3,
                            ),
                        ),
                    },
                },
                op: And,
                right: BinaryOp {
                    left: ColumnRef {
                        database: None,
                        table: None,
                        column: Ident {
                            value: "b",
                        },
                    },
                    op: Lt,
                    right: ColumnRef {
                        database: None,
                        table: None,
                        column: Ident {
                            value: "c",
                        },
                    },
                },
            },
        ),
        group_by: [
            ColumnRef {
                database: None,
                table: None,
                column: Ident {
                    value: "a",
                },
            },
            ColumnRef {
                database: None,
                table: None,
                column: Ident {
                    value: "c",
                },
            },
        ],
    },
    order_by: [
        OrderByExpr {
            expr: ColumnRef {
                database: None,
                table: None,
                column: Ident {
                    value: "a",
                },
            },
            asc: None,
        },
        OrderByExpr {
            expr: ColumnRef {
                database: None,
                table: None,
                column: Ident {
                    value: "b",
                },
            },
            asc: Some(
                false,
            ),
        },
    ],
    limit: Some(
        Literal(
            UnsignedInteger(
                1,
            ),
        ),
    ),
    offset: Some(
        Literal(
            UnsignedInteger(
                2,
            ),
        ),
    ),
}
```

## References
- [SQL92 Standard](https://www.contrib.andrew.cmu.edu/~shadow/sql/sql1992.txt)
- [Antlr Grammars for SQL](https://github.com/antlr/grammars-v4/tree/master/sql)
- [PostgreSQL Doc](https://www.postgresql.org/docs/16/sql.html)
- [BNF Grammars for SQL-92, SQL-99 and SQL-2003](https://ronsavage.github.io/SQL/)
- [Queries - Microsoft SQL Doc](https://learn.microsoft.com/en-us/sql/t-sql/queries/queries)
- [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [手写一个Parser - 代码简单而功能强大的Pratt Parsing](https://zhuanlan.zhihu.com/p/471075848)
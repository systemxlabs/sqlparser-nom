# sqlparser-nom 
A experimental SQL query parser using nom.

- [x] Query
  - [x] Select
  - [x] From
  - [x] Where
  - [x] Order by
  - [x] Limit
  - [x] CTE
  - [x] Group by
  - [x] Having
  - [x] Aggregate
  - [x] Window
- [x] Pratt Parsing
- [ ] Friendly error info

## Example
```sql
select a, count(*)
from (select * from t1) as t2
join t3 on t2.a = t3.a
left join t4 on t3.b = t4.b
where a > ((1 + 2) * 3) and b < c 
group by a, c 
having count(*) > 5
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
                Function {
                    name: Ident {
                        value: "count",
                    },
                    distinct: false,
                    args: [
                        Wildcard,
                    ],
                },
            ),
        ],
        from: Some(
            Join {
                op: LeftOuter,
                condition: On(
                    BinaryOp {
                        left: ColumnRef {
                            database: None,
                            table: Some(
                                Ident {
                                    value: "t3",
                                },
                            ),
                            column: Ident {
                                value: "b",
                            },
                        },
                        op: Eq,
                        right: ColumnRef {
                            database: None,
                            table: Some(
                                Ident {
                                    value: "t4",
                                },
                            ),
                            column: Ident {
                                value: "b",
                            },
                        },
                    },
                ),
                left: Join {
                    op: Inner,
                    condition: On(
                        BinaryOp {
                            left: ColumnRef {
                                database: None,
                                table: Some(
                                    Ident {
                                        value: "t2",
                                    },
                                ),
                                column: Ident {
                                    value: "a",
                                },
                            },
                            op: Eq,
                            right: ColumnRef {
                                database: None,
                                table: Some(
                                    Ident {
                                        value: "t3",
                                    },
                                ),
                                column: Ident {
                                    value: "a",
                                },
                            },
                        },
                    ),
                    left: Subquery {
                        subquery: SelectStatement {
                            body: Select {
                                projection: [
                                    Wildcard,
                                ],
                                from: Some(
                                    BaseTable {
                                        name: TableName {
                                            database: None,
                                            table: Ident {
                                                value: "t1",
                                            },
                                        },
                                        alias: None,
                                    },
                                ),
                                selection: None,
                                group_by: [],
                                having: None,
                            },
                            order_by: [],
                            limit: None,
                            offset: None,
                        },
                        alias: Some(
                            Ident {
                                value: "t2",
                            },
                        ),
                    },
                    right: BaseTable {
                        name: TableName {
                            database: None,
                            table: Ident {
                                value: "t3",
                            },
                        },
                        alias: None,
                    },
                },
                right: BaseTable {
                    name: TableName {
                        database: None,
                        table: Ident {
                            value: "t4",
                        },
                    },
                    alias: None,
                },
            },
        ),
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
        having: Some(
            BinaryOp {
                left: Function {
                    name: Ident {
                        value: "count",
                    },
                    distinct: false,
                    args: [
                        Wildcard,
                    ],
                },
                op: Gt,
                right: Literal(
                    UnsignedInteger(
                        5,
                    ),
                ),
            },
        ),
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
- [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs)
- [databend](https://github.com/datafuselabs/databend/tree/main/src/query/ast)
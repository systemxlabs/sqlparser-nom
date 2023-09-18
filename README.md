# sqlparser-lalrpop 
A experimental SQL query parser using LALRPOP.

Query
- [ ] Select
- [ ] Distinct
- [ ] From
- [x] Where
- [x] Order by
- [x] Limit
- [ ] CTE
- [ ] Into
- [x] Group by
- [ ] Having
- [ ] Window

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
Select(
    SelectStatement {
        body: Select {
            projection: [
                UnnamedExpr(
                    Identifier(
                        Ident {
                            value: "a",
                        },
                    ),
                ),
                UnnamedExpr(
                    CompoundIdentifier(
                        [
                            Ident {
                                value: "t",
                            },
                            Ident {
                                value: "b",
                            },
                        ],
                    ),
                ),
                UnnamedExpr(
                    Identifier(
                        Ident {
                            value: "c",
                        },
                    ),
                ),
            ],
            from: Ident {
                value: "t",
            },
            where_clause: Some(
                BinaryOp {
                    left: BinaryOp {
                        left: Identifier(
                            Ident {
                                value: "a",
                            },
                        ),
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
                        left: Identifier(
                            Ident {
                                value: "b",
                            },
                        ),
                        op: Lt,
                        right: Identifier(
                            Ident {
                                value: "c",
                            },
                        ),
                    },
                },
            ),
            group_by: [
                Identifier(
                    Ident {
                        value: "a",
                    },
                ),
                Identifier(
                    Ident {
                        value: "c",
                    },
                ),
            ],
        },
        order_by: [
            OrderByExpr {
                expr: Identifier(
                    Ident {
                        value: "a",
                    },
                ),
                asc: None,
            },
            OrderByExpr {
                expr: Identifier(
                    Ident {
                        value: "b",
                    },
                ),
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
    },
)
```

## References
- [SQL92 Standard](https://www.contrib.andrew.cmu.edu/~shadow/sql/sql1992.txt)
- [Antlr Grammars for SQL](https://github.com/antlr/grammars-v4/tree/master/sql)
- [PostgreSQL Doc](https://www.postgresql.org/docs/16/sql.html)
- [BNF Grammars for SQL-92, SQL-99 and SQL-2003](https://ronsavage.github.io/SQL/)
- [Queries - Microsoft SQL Doc](https://learn.microsoft.com/en-us/sql/t-sql/queries/queries)
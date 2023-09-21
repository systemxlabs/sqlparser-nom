# sqlparser-nom 
A experimental SQL query parser using nom.

- [ ] Query
  - [ ] Select
  - [ ] From
  - [ ] Where
  - [ ] Order by
  - [ ] Limit
  - [ ] CTE
  - [ ] Into
  - [ ] Group by
  - [ ] Having
  - [ ] Window
- [x] Pratt Parsing
- [ ] Friendly error info

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
```

## References
- [SQL92 Standard](https://www.contrib.andrew.cmu.edu/~shadow/sql/sql1992.txt)
- [Antlr Grammars for SQL](https://github.com/antlr/grammars-v4/tree/master/sql)
- [PostgreSQL Doc](https://www.postgresql.org/docs/16/sql.html)
- [BNF Grammars for SQL-92, SQL-99 and SQL-2003](https://ronsavage.github.io/SQL/)
- [Queries - Microsoft SQL Doc](https://learn.microsoft.com/en-us/sql/t-sql/queries/queries)
- [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
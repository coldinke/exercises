# SQL

## Part I: Querying Data

### SELECT

One of the most common tasks, when you work with the database, is to retrieve data from tables using the `SELECT` statement.

The `SELECT` statement has the following clauses:
- Select distinct rows using `DTSTINCT` operator.
- Sort rows using `ORDER BY` clause.
- Filter rows using `WHERE` clause.
- Select a subset of rows from a table using `LIMIT` or `FETCH` clause.
- Group rows into groups using `GROUP BY` clause.
- Filter groups using `HAVING` clause.
- Join with other tables using joins such as `INNER JOIN`, `LEFT JOIN`, `FULL OUTER`, `JOIN`, `CROSS JOIN` clauses.
- Perform set operating using `UNION`, `INTERSECT`, and `EXCEPT`.

PostgreSQL SELECT statement syntax

```sql
SELECT
   select_list
FROM
   table_name;
```

In this syntax:
- First, specify a select list that can be a column or a list of columns in a table from which you want to retrieve dadta. If you specify a list of columns, you need to palce a comma(`,`) between two columns to separate them. If you want to select data from all the columns of the table, you can use an asterisk(`*`) shorthand instead of specifying all the column names. The select list may also contain expressions of literal values.

- Second, provide the name of the table from which you want to query data after the `FROM` keyword.


The `FROM` clause is optional. If you are not querying data from any table, you can omit the `FROM` clause in the `SELECT` statement.

PostgreSQL evaluates the `FROM` clause before the `SELECT` clause in the `SELECT` statement:

![alt text](https://www.postgresqltutorial.com/wp-content/uploads/2020/07/PostgreSQL-Select.png)


Instead of listing all columns in the `SELECT` clause individually, we can use the asterisk(`*`) to make the query shorter.

However, using the asterisk(`*`) in the `SELECT` statement is considered a bad practice when you embed SQL statements in the application code.
- Database performance. Suppose you have a table with many columns and substantial data, the `SELECT` statement with the asterisk(`*`) shorthand will select data from all the columns of the table, potentially retrieving more data than required for the application.
- Application performance. Retrieving unnecessary data from the database increases the traffic between the PostgreSQL server and the application server. Consequently, this can result in slower response times and reduced scalability for your applications.


### Column Alias

A column alias allows you to assign a column or an expression in the select list of a `SELECT` statement a temporary name. The column alias exists temporarily during the execution of the query.

The follwing illustrates the syntax of using a column alias:

```sql
SELECT column_name AS alias_name
FROM table_name;
```

In this syntax, the `column_name` is assigned an alias `alias_name`. The `AS` keyword is optional so you can omit it like this:

```sql
SELECT column_name alias_name
FROM table_name;
```

The following syntax illustates how to set an alias for an expression in the `SELECT` clause:

1. Assigning a column alias to a column example

```sql
SELECT expression AS alias_name
FROM table_name;
```

2. Assigning a column alias to an expression example

```sql
SELECT 
    first_name || ' ' || last_name
FROM
    customer;
```

Note that in PostgreSQL, you use the `||` as the concatenating operator that concatenatenates one or more strings into a single string.

The heading of the column is not meaningful `?column?`.

To fix this, you can assign the expression `first_name || ' ' || last_name` a column alias e.g., `full_name`:

```sql
SELECT
    first_name || ' ' || last_name AS full_name
FROM
    customer;
```

3. Column aliases that contain spaces

```sql
SELECT 
    first_name || ' ' || last_name "full name"
FROM
    customer;
```


### ORDER BY

When you query data from a table, the `SELECT` statement returns rows in an unspecified order. To sort the rows of the result set, you use the `ORDER BY` clause in the `SELECT` statement.

The `ORDER BY` clause allows you to sort rows returned by a `SELECT` clause in ascending or sdescending order based on a sort expression.

The following illustrates the syntax of the `ORDER BY` cluase:

```sql
SELECT
    select_list
FROM
    table_name
ORDER BY
    sort_expression1 [ASC | DESC],
    sort_expression2 [ASC | DESC],
    ...;
```

In this syntax:
- First, specify a sort expression, which can be a column or expression, that you want to sort after the `ORDER BY` keywords. If you want to sort the result set based on multiple columns or expressions, you need to place a common(`,`) between two columns or expressions to separater them.
- Second, you use the `ASC` option to sort rows in ascending order and the `DESC` option to sort rows in descending order. If you omit the `ASC` or `DESC` option, the `ORDER BY` uses `ASC` by default.

PostgreSQL evaluates the clauses in the `SELECT` statement in the following order: `FROM`, `SELECT`, and `ORDER BY`:

![alt text](https://www.postgresqltutorial.com/wp-content/uploads/2020/07/PostgreSQL-ORDER-BY.png)


### SELECT DISTINCT

The `SELECT DISTINCT` removes duplicate rows from a result set. The `SELECT DISTINCT` clause retains one row for each group of duplicates.

The `SELECT DISTINCT` clause can be applied to one or more columns in the select list of the `SELECT` statement.

The following illustrates the syntax of the `DISTINCT` clause:

```sql
SELECT 
    DISTINCT column1
FROM
    table_name;
```

If you specify multiple columns, the `SELECT DISTINCT` clause will evaluate the duplicate based on the combination of values in these colunmns. For example:

```sql
SELECT
    DISTINCT column1, column2
FROM
    table_name;
```


## Part II: Filtering Data

### WHERE 

The `SELECT` statement returns all rows from one or more columns in a table. To retrieve rows that **satisfy** a specified condition, you use a `WHERE` clause.

The syntax of the PostgreSQL, `WHERE` clause is as fllows:

```sql
SELECT
    select_list
FROM
    table_name
WHERE
    condition
ORDER BY
    sort_expression;
```

In this syntax, you place the `WHERE` clause right after the `FROM` clause of the `SELECT` statement.

The `WHERE` clause uses the `condition` to filter the rows returned from the `SELECT` clause. 

The `condition` is a boolean expression that evaluates to true, false or unknown.

The query returns only rows that satisfy the `condition` in the `WHERE` clause. In other words, the query will include only rows that cause the `condition` evaluates to **true** in the result set.

PostgreSQL evaluates the `WHERE` clause after the `FROM` clause but before the `SELECT` and `ORDER BY` clause:

![where progress](https://blog-pricture.obs.cn-east-3.myhuaweicloud.com/image/where_progress.png)

Beside the `SELECT` statement, you can use the `WHERE` clause in the `UPDATE` and `DELETE` statement to specify rows to update and delete.


### AND operator

In postgreSQL, a boolean value can have one of three values: `true`, `false`, and `null`.

PostgreSQL uses `true`, `'t'`, `'true'`, `'y'`, `'yes'`, `'1'` to represent `true` and `false`, `'f'`, `'false'`, `'n'`, `'no'`, and `'0'` to represent `false`.

Here's the basic syntax of the `AND` operator:
```sql
expression1 AND expression2
```

In this syntax, `expression1` and `expression2` are boolean expressions that evaluate to `true`, `false`, or `null`.

The following table shows the result of the `AND` operator when combining `true`, `false`, and `null`.

| AND   | True  | False | Null  |
|-------|-------|-------|-------|
| True  | True  | False | Null  |
| False | False | False | False |
| Null  | Null  | False | Null  |


### OR operator

The `OR` operator is a logical operator that combines multiple boolean expressions. Here's the basic syntax  of the `OR` operator:

```sql
expression1 OR expression2
```

In this syntax, `expression1` and `expression2` are boolean expressions that evaluate to `true`, `false`, or `null`.

The `OR` operator returns `true` only if any of the expressions is `true`. It returns `false` if both expressions are false. Otherwise, it returns null.

The following talbe shows the result of the `OR` operator when combining `true`, `false` and `null`.


|  OR   | True  | False | Null  |
|-------|-------|-------|-------|
| True  | True  | True  | True  |
| False | True  | False | Null  |
| Null  | True  | Null  | Null  |


### LIMIT

PostgreSQL `LIMIT` is an optional clause of the `SELECT` statement that constrains the number of rows returned by the query.

Here's the basic syntax of the `LIMIT` clause:

```sql
SELECT
    select_list
FROM
    table_name
ORDER BY
    sort_expression
LIMIT
    row_count;
```

The statement returns `row_count` rows generated by the query. If the `row_count` is zero, the query **returns an empty set**. If the `row_count` is `NULL`, the query **returns the same result set as it does not have the `LIMIT` clause**.


If you want to **skip** a number of rows before returning the `row_count` rows, you can use `OFFSET` clause placed after the `LIMIT` clause:

```sql
SELECT
    select_list
FROM
    table_name
ORDER BY
    sort_expression
LIMIT
    row_count
OFFSET
    row_to_skip;
```

The statement first skip `row_to_skip` rows before returnning `row_count` rows generated by the query.

If the `row_to_skip` is zero, the statement will work like it **doesn't have the `OFFSET` clause**. 

It's important to note the PostgreSQL evaluates the **`OFFSET`** clause before the `LIMIT` clause.


### FETCH

The `LIMIT` clause is widely used by many Relation Database Management Systems such as MySQL, H2 and HSQLDB. However, the `LIMIT` clause is not a SQL standard.

To conform with the SQL standard, PostgreSQL supports the `FETCH` caluse to skip a certian number of rows and then fetch a specific number of rows.

The following illustrates the syntax of the PostgreSQL `FETCH` clause:

```sql
OFFSET row_to_skip { ROW | ROWS }
FETCH { FIRST | NEXT } [ row_count ] { ROW | ROWS } ONLY
```

In this syntax:

First, specify the number of rows to skip (`row_to_skip`) after the `OFFSET` keyword. The start is an integer that is zero or positive. It defaults to 0, meaning the query will skip no rows.

If the `row_to_skip` is higher than the number of rows in the table, the query return no rows.

Second, provides the number of rows to retrieve(`row_count`) in the `FETCH` clause. The `row_count` must be an integer 1 or greater. The `row_count` defaulst to 1.

The `Row` is the synonym for `ROWS`, `FIRST` is the synonym for `NEXT` so you can use them interchangeably.

The `Fetch` clause is functionally equivalent to the `LIMIT` clause. If you plan to make your application compatible with other database systems, you should use the `FETCH` clause because it follows the standard SQL.


### IN

The `IN` operator allows you to check whether a value matches any value in a **list of values**.

Here's the basic syntax of the `IN` operator:

```sql
value IN (value1, value2, ...)
```

The `IN` operator returns true if the `value` is equal to any value in the list such as `value1` and `value2`.

The list of values can be a list of **literal values including numbers and strings**.

In addition to literal values, the `IN` operator also accepts a list of values returned from a query.

Functionally, the `IN` operator is equivalent to combining multiple boolean expressions with the OR operators:

```sql
value = value1 OR value = value2 OR ...
```


### NOT IN

To negate the `IN` operator, you use the `NOT IN` operator. Here's the basic syntax of the `NOT IN` operator:

```sql
value NOT IN (value1, value2, ...)
```

The `NOT IN` operator returns `true` if the `value` is not equal to any value in the list such as `value1` and `value2`; otherwise, the `NOT IN` operator returns `false`.

The `NOT IN` operator is equivalent to a combination of multiple boolean expressions with the AND operators:

```sql
value <> value1 AND value <> value2 AND ...
```


### BETWEEN

The `BETWEEN` operator allows you to check if a value falls within a range of values.

The basic syntax of the `BETWEEN` operator is as fllows:

```sql
value BETWEEN low AND high;
```

If the `value` is greater than or equal to the `low` value and less than or equal to the `high` value, the `BETWEEN` operator returns `true`; otherwise, it returns `false`.

You can rewrite the `BETWEEN` operator by using the greater than or equal (`>=`) and less than or equal to (`<=`) operators and the logical AND operator:

```sql
value >= low AND value <= high
```

If you want to check if a value is outside a specify range, you can use the `NOT BETWEEN` operator as follows: 

```sql
value NOT BETWEEN low AND high
```

The follwing expression is equivalent the expression that uses the `NOT BETWEEN` operators:

```sql
value < low OR value > high
```


### Like

You construct a pattern by combining literal values with wildcard characters and using the `LIKE` or `NOT LIKE` operator to find the matches.

PosgreSQL offers two wildcards:
- Percent sign (`%`) matches any sequence of zero or more characters.
- Underscore sign (`_`) matches any single character.

Here's the basic syntax of the `LIKE` operator:

```sql
value LIKE pattern
```

The `LIKE` operator returns `true` if the `value` matches the `pattern`. To negate the `LIKE` operator, you use the `NOT` operator as follows:

```sql
value NOT LIKE pattern
```

The `NOT LIKE` operator returns `true` when the `value` does not match the `pattern`.

If the pattern does not contian any wildcard character, the `LIKE` operator behaves like the equal (`=`) operator.

PostgreSQL `ILIKE` operator, which is similar to the `LIKE` operator, but allows for **case-insensitive matching**. 

PostgreSQL also provides some operators that mirror the functionality of `LIKE`, `NOT LIKE`, `ILIKE`, `NOT ILIKE`, as shown in the follow table:
| Operator | Equivalent |
| -------- | ---------- |
| ~~       | LIKE       |
| ~~*      | ILIKE      |
| !~~      | NOT LIKE   |
| !~~*     | NOT ILIKE  |


### IS NULL

In the database world, NULL means missing information or not applicable. NULL is not a value, therefore, you cannot compare it with other values like numbers or strings.

To comparison of NULL with a value will always result in NULL. Additionally, NULL is not equal to NULL so the following expression returns NULL.

To check if a value is NULL or not, you can use the `IS NULL` operator.

Here's the basic syntax of the `IS NULL` operator:

```sql
value IS NULL
```

The `IS NULL` operator returns true if the `value` is NULL or false otherwise.

To negate the `IS NULL` operator, you use the `IS NOT NULL` operator.

```sql
value IS NOT NULL
```

The `IS NOT NULL` operator returns true if the value is not NULL or fase otherwise.


## Part III: Joining Multiple Tables

PostgreSQL join is used to combine columns from one(self-join) or more tables based on the values of the common columns between related tables. The common columns are typically the primary key column of the first table and the foreign key column of the second table.



## Part IV: Grouping Data


## Part V: Set Operations

## Part VI: Grouping sets, Cubes, and Rollups


## References

PostgreSQL Tutorial: https://www.postgresqltutorial.com/
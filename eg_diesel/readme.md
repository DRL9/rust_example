```zsh
# 初始化
diesel setup

# 运行完后，填写 migrations/$ts_create_posts 下面的 down.sql, up.sql
diesel migration generate create_posts

# 生成 schema.rs
diesel migration run

# 回退，会调用 down.sql
diesel migration revert
```

## TODO
- [ ] 联表查询
- [ ] 从数据库生成 schema
- [ ] 从数据库生成查询 model
- [ ] transation
- [ ] 如果使用连接池
- [ ] 连接mysql
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
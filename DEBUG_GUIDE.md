# 数据库连接调试指南

## 已添加的调试功能

### 1. 详细的日志记录
- **API 端点日志** (`upsert_database`): 记录请求开始、连接测试、存储等步骤
- **连接测试日志** (`test_connection`): 记录每个连接步骤的详细信息
- **存储操作日志** (`store_connection`): 记录验证和数据库操作
- **错误日志**: 所有错误都会被详细记录

### 2. 超时机制
- **连接超时**: 15秒（如果 PostgreSQL 不可达，会在15秒后返回错误）
- **查询超时**: 5秒（测试查询的超时时间）
- **总超时**: 20秒（整个请求的超时时间）

### 3. 日志级别
- 使用 `DEBUG` 级别，显示文件名和行号
- 记录每个步骤的执行时间

## 如何查看日志

### 方法1: 查看服务器窗口
服务器应该在一个新窗口中运行，你可以看到实时的日志输出。

### 方法2: 手动启动服务器
```powershell
cd backend
$env:SQLITE_DB_PATH = ".\test.db"
$env:PORT = "8080"
cargo run
```

### 方法3: 使用测试脚本
```powershell
.\test-db-with-logs.ps1
```

## 日志输出示例

当你尝试添加数据库时，你应该看到类似以下的日志：

```
[test_connection] Starting connection test for URL: postgresql://...
[test_connection] Detected PostgreSQL connection string
[test_connection] Parsing PostgreSQL connection options...
[test_connection] Connection options parsed successfully
[test_connection] Connecting to: postgresql://postgres:***@localhost:5432/chat
[test_connection] Connection timeout set to 15 seconds
[test_connection] Attempting to connect to PostgreSQL database...
[test_connection] Connection attempt took X.XX seconds
```

如果连接失败，你会看到：
- 连接错误的具体信息
- 超时信息（如果超过15秒）
- 每个步骤的执行时间

## 常见问题排查

### 1. 连接超时
- **可能原因**: PostgreSQL 服务未运行
- **检查**: `Get-Service postgresql*` (Windows) 或 `systemctl status postgresql` (Linux)
- **解决**: 启动 PostgreSQL 服务

### 2. 连接被拒绝
- **可能原因**: PostgreSQL 监听地址或端口不正确
- **检查**: 确认 PostgreSQL 配置中的 `listen_addresses` 和 `port`
- **解决**: 修改 PostgreSQL 配置或使用正确的连接字符串

### 3. 认证失败
- **可能原因**: 用户名或密码错误
- **检查**: 日志中会显示连接信息（密码会被隐藏）
- **解决**: 使用正确的用户名和密码

### 4. 数据库不存在
- **可能原因**: 指定的数据库不存在
- **检查**: 连接字符串中的数据库名称
- **解决**: 创建数据库或使用正确的数据库名称

## 下一步调试

如果问题仍然存在，请：
1. 查看服务器窗口中的详细日志
2. 确认 PostgreSQL 服务正在运行
3. 测试直接连接: `psql -h localhost -p 5432 -U postgres -d chat`
4. 检查防火墙设置
5. 查看 PostgreSQL 日志文件


# biox

一个生物信息学工具箱（Rust 实现）。

## 构建

```bash
cargo build --release
```

编译产物位于 `target/release/biox`。

## 发布

推送 `v*` 形式的 tag 时，GitHub Actions 会自动编译多平台二进制并发布到 Releases：

```bash
git tag v0.1.0
git push origin v0.1.0
```

工作流定义见 `.github/workflows/release.yml`，覆盖以下平台：

- Linux：`x86_64-unknown-linux-gnu`、`x86_64-unknown-linux-musl`、`aarch64-unknown-linux-gnu`
- macOS：`x86_64-apple-darwin`、`aarch64-apple-darwin`
- Windows：`x86_64-pc-windows-msvc`

## 使用

```bash
biox <子命令>
```

### 子命令

| 子命令 | 说明 |
| --- | --- |
| `tsv2md` | 将 TSV（制表符分隔）文本转换为 Markdown 表格 |
| `csv2md` | 将 CSV（逗号分隔）文本转换为 Markdown 表格 |

## tsv2md 命令

将 TSV 文本从文件或标准输入读取，并转换为 Markdown 表格。

### 语法

```bash
biox tsv2md [FILE]
```

- 指定 `FILE` 时，从该文件读取 TSV 内容；
- 省略 `FILE` 时，从标准输入（stdin）读取。

### 示例

从文件读取：

```bash
biox tsv2md data.tsv
```

从标准输入读取：

```bash
printf 'Name\tAge\nAlice\t30\nBob\t25\n' | biox tsv2md
```

输出：

```markdown
| Name  | Age |
|-------|-----|
| Alice | 30  |
| Bob   | 25  |
```

### 转换规则

- 第一个非空行作为表头；
- 忽略空行；
- 各行列数不一致时，以最宽的行补齐空单元格；
- 单元格内的竖线 `|` 会被转义为 `\|`。

## csv2md 命令

将 CSV 文本从文件或标准输入读取，并转换为 Markdown 表格。

### 语法

```bash
biox csv2md [FILE]
```

- 指定 `FILE` 时，从该文件读取 CSV 内容；
- 省略 `FILE` 时，从标准输入（stdin）读取。

### 示例

从文件读取：

```bash
biox csv2md data.csv
```

从标准输入读取：

```bash
printf 'Name,Age\nAlice,30\nBob,25\n' | biox csv2md
```

输出：

```markdown
| Name  | Age |
|-------|-----|
| Alice | 30  |
| Bob   | 25  |
```

### 转换规则

- 第一行作为表头；
- 各行列数不一致时，以最宽的行补齐空单元格；
- 单元格内的竖线 `|` 会被转义为 `\|`。

## 帮助

查看全局帮助：

```bash
biox --help
```

查看子命令帮助：

```bash
biox tsv2md --help
biox csv2md --help
```

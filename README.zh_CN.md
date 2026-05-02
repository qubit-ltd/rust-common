# Qubit Common

[![CircleCI](https://circleci.com/gh/qubit-ltd/rs-common.svg?style=shield)](https://circleci.com/gh/qubit-ltd/rs-common)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-common/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-common?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-common.svg?color=blue)](https://crates.io/crates/qubit-common)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Qubit 相关项目提供语言级基础工具和数据类型支持的综合性 Rust 工具库。

## 概述

Qubit Common 旨在提供 Rust 应用程序中常用的基础语言级工具。它提供强大的参数验证、全面的数据类型定义以及遵循 Rust 惯用法和最佳实践的核心工具函数。

## 特性

### 🔧 **数据类型系统**
- **通用数据类型枚举**：支持所有基本 Rust 类型和常见第三方类型的全面 `DataType` 枚举
- **编译时类型映射**：用于编译时类型到数据类型查询的 `DataTypeOf` 特征
- **序列化支持**：所有数据类型的内置 JSON/YAML 序列化
- **数据转换**：可复用的 `DataConverter` 和 `DataConverters`，用于在 `DataType` 支持的 Rust 类型之间转换单个运行时值或批量值

### 🛡️ **参数验证**
- **数值验证**：范围检查、相等性比较和边界验证
- **字符串验证**：模式匹配、长度约束和格式验证
- **集合验证**：大小限制、元素约束和空值检查
- **可选值验证**：空值安全性和可选值处理
- **方法链式调用**：流畅的 API 设计，支持可读的验证链

### 🎯 **核心工具**
- **错误处理**：具有详细上下文的全面错误类型
- **装箱错误辅助工具**：`BoxError`、`BoxResult`、`DynError` 和 `IntoBoxError`，用于类型擦除的集成错误
- **状态验证**：应用程序状态检查和验证
- **条件检查**：灵活的条件和断言工具

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-common = "0.7.0"
```

## 快速开始

### 数据类型使用

```rust
use qubit_common::lang::{DataType, DataTypeOf};

// 获取数据类型信息
let data_type = DataType::Int32;
assert_eq!(data_type.as_str(), "int32");

// 编译时类型映射
assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert_eq!(String::DATA_TYPE, DataType::String);

// 序列化
let json = serde_json::to_string(&DataType::Float64).unwrap();
assert_eq!(json, "\"float64\"");
```

### 数据转换

```rust
use std::time::Duration;

use qubit_common::lang::converter::{
    DataConversionResult,
    DataConverter,
    DataConverters,
    DataListConversionResult,
};

fn read_settings() -> DataConversionResult<(u16, bool, Duration)> {
    let port = DataConverter::from("8080").to::<u16>()?;
    let enabled = DataConverter::from("true").to::<bool>()?;
    let timeout = DataConverter::from("1500000000ns").to::<Duration>()?;

    Ok((port, enabled, timeout))
}

fn read_ports(values: &[String]) -> DataListConversionResult<Vec<u16>> {
    DataConverters::from(values).to_vec()
}
```

`DataConverter` 是轻量级的源值包装器。它通过 `From` 实现接受借用或
拥有所有权的输入；当转换不支持、源值为空、内容无效或超出目标类型范围时，
会返回 `DataConversionError`。

`DataConverters` 将同样的单值转换规则应用到借用切片、借用向量、拥有所有权的
向量或任意迭代器上。它按源数据顺序返回转换结果，并在某个元素转换失败时报告
从零开始的失败元素索引。

### 参数验证

```rust
use qubit_common::lang::argument::{
    NumericArgument, StringArgument, CollectionArgument, ArgumentResult
};
use regex::Regex;

fn process_user_input(
    age: i32,
    username: &str,
    tags: &[String],
) -> ArgumentResult<()> {
    // 数值验证
    let age = age.require_in_closed_range("age", 0, 150)?;

    // 字符串验证（链式调用）
    let username_pattern = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{2,19}$").unwrap();
    let username = username
        .require_non_blank("username")?
        .require_match("username", &username_pattern)?;

    // 集合验证
    let tags = tags
        .require_non_empty("tags")?
        .require_length_at_most("tags", 10)?;

    println!("年龄: {}, 用户名: {}, 标签数量: {}", age, username, tags.len());
    Ok(())
}
```

### 状态和条件检查

```rust
use qubit_common::lang::argument::{
    check_argument_with_message, check_bounds, check_state_with_message, ArgumentResult
};

fn process_data(
    offset: usize,
    length: usize,
    total_length: usize,
    items: &[String],
) -> ArgumentResult<()> {
    // 基本参数检查
    check_argument_with_message(total_length > 0, "total_length must be positive")?;

    // 状态验证
    check_state_with_message(!items.is_empty(), "items cannot be empty")?;

    // 边界检查
    check_bounds(offset, length, total_length)?;

    Ok(())
}
```

### 装箱错误辅助工具

`BoxError` 和 `BoxResult<T>` 适用于有意擦除具体错误类型的场景，例如回调、
小型可执行入口、集成胶水代码以及在外层具体错误中保存 source error。
`DynError` 命名统一的 `dyn Error + Send + Sync + 'static` 对象约束，
`IntoBoxError` 则在 `map_err` 或手动构造 `Err` 时提供显式转换方法。

如果公共 API 的调用方需要按错误类型或变体做恢复，应优先返回具体错误类型。
当调用方只需要传播、记录或保存原始错误来源时，再使用这些装箱辅助工具。

```rust
use qubit_common::lang::error::{BoxResult, IntoBoxError};

fn parse_port(text: &str) -> BoxResult<u16> {
    text.parse::<u16>()
        .map_err(|error| error.into_box_error())
}

let port = parse_port("8080").expect("valid port should parse");
assert_eq!(port, 8080);
assert!(parse_port("not-a-port").is_err());
```

## 支持的数据类型

完整变体见 [`DataType`](https://docs.rs/qubit-common/latest/qubit_common/lang/enum.DataType.html)；字符串形式由 `as_str()` 给出（例如 `int32`、`instant`、`stringmap`）。下文在适用处标明是否实现 [`DataTypeOf`](https://docs.rs/qubit-common/latest/qubit_common/lang/trait.DataTypeOf.html)。

### 基本类型
- **整数**：`i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
- **平台相关整数**：`isize`, `usize`（对应 `DataType::IntSize`、`DataType::UIntSize`）
- **浮点数**：`f32`, `f64`
- **其他**：`bool`, `char`, `String`

### 日期/时间类型
- **Chrono**：`NaiveDate` → `DataType::Date`，`NaiveTime` → `Time`，`NaiveDateTime` → `DateTime`，`DateTime<Utc>` → `Instant`（UTC 时刻，语义上类似 Java 的 `Instant`）

### 大数类型
- **任意精度**：`BigInt`, `BigDecimal`

### 时长、映射、JSON 与 URL
- **时长**：`std::time::Duration` → `DataType::Duration`
- **字符串映射**：`HashMap<String, String>` → `DataType::StringMap`
- **JSON 值**：`serde_json::Value` → `DataType::Json`
- **URL**：`url::Url` → `DataType::Url`，已实现 [`DataTypeOf`](https://docs.rs/qubit-common/latest/qubit_common/lang/trait.DataTypeOf.html)；crate 直接依赖 [`url`](https://crates.io/crates/url)，在类型层面固定约定。

## API 参考

### 数据类型
- [`DataType`](https://docs.rs/qubit-common/latest/qubit_common/lang/enum.DataType.html) - 通用数据类型枚举
- [`DataTypeOf`](https://docs.rs/qubit-common/latest/qubit_common/lang/trait.DataTypeOf.html) - 编译时类型映射特征

### 数据转换
- [`DataConverter`](https://docs.rs/qubit-common/latest/qubit_common/lang/converter/enum.DataConverter.html) - 运行时值转换包装器
- [`DataConverters`](https://docs.rs/qubit-common/latest/qubit_common/lang/converter/struct.DataConverters.html) - 批量运行时值转换适配器
- [`DataConversionError`](https://docs.rs/qubit-common/latest/qubit_common/lang/converter/enum.DataConversionError.html) - 不支持或无效转换的错误类型
- [`DataConversionResult`](https://docs.rs/qubit-common/latest/qubit_common/lang/converter/type.DataConversionResult.html) - 转换 API 返回的结果别名
- [`DataListConversionError`](https://docs.rs/qubit-common/latest/qubit_common/lang/converter/struct.DataListConversionError.html) - 包含失败批量元素索引的错误类型
- [`DataListConversionResult`](https://docs.rs/qubit-common/latest/qubit_common/lang/converter/type.DataListConversionResult.html) - 批量转换 API 返回的结果别名

### 参数验证
- [`NumericArgument`](https://docs.rs/qubit-common/latest/qubit_common/lang/argument/trait.NumericArgument.html) - 数值验证方法
- [`StringArgument`](https://docs.rs/qubit-common/latest/qubit_common/lang/argument/trait.StringArgument.html) - 字符串验证方法
- [`CollectionArgument`](https://docs.rs/qubit-common/latest/qubit_common/lang/argument/trait.CollectionArgument.html) - 集合验证方法
- [`OptionArgument`](https://docs.rs/qubit-common/latest/qubit_common/lang/argument/trait.OptionArgument.html) - 可选值验证方法

### 核心函数
- [`check_argument`](https://docs.rs/qubit-common/latest/qubit_common/lang/argument/fn.check_argument.html) - 基本参数验证
- [`check_state`](https://docs.rs/qubit-common/latest/qubit_common/lang/argument/fn.check_state.html) - 状态验证
- [`check_bounds`](https://docs.rs/qubit-common/latest/qubit_common/lang/argument/fn.check_bounds.html) - 边界检查

### 错误辅助工具
- [`DynError`](https://docs.rs/qubit-common/latest/qubit_common/lang/error/type.DynError.html) - 统一的动态错误对象约束
- [`BoxError`](https://docs.rs/qubit-common/latest/qubit_common/lang/error/type.BoxError.html) - 拥有所有权的装箱动态错误
- [`BoxResult`](https://docs.rs/qubit-common/latest/qubit_common/lang/error/type.BoxResult.html) - 使用 `BoxError` 的结果别名
- [`IntoBoxError`](https://docs.rs/qubit-common/latest/qubit_common/lang/error/trait.IntoBoxError.html) - 将具体错误显式装箱的转换 trait

## 错误处理

参数验证函数返回 `ArgumentResult<T>`，它是 `Result<T, ArgumentError>` 的别名。
`ArgumentError` 保存一段可读的验证错误消息，并实现标准错误 trait。

```rust
use qubit_common::lang::argument::{ArgumentError, ArgumentResult};

fn validate_input(value: i32) -> ArgumentResult<i32> {
    if value > 0 {
        Ok(value)
    } else {
        Err(ArgumentError::new("value must be positive"))
    }
}

match validate_input(0) {
    Ok(result) => println!("验证通过: {result}"),
    Err(error) => eprintln!("验证失败: {error}"),
}
```

## 依赖项

- **serde**：序列化支持
- **serde_json**：JSON 序列化
- **bigdecimal**：任意精度十进制算术
- **chrono**：日期和时间处理
- **num-bigint**：大整数支持
- **num-traits**：数值转换支持
- **regex**：模式匹配
- **url**：解析后的 URL 类型（`url::Url`），与 `DataType::Url` 绑定

## 测试与代码覆盖率

本项目保持全面的测试覆盖，对所有功能进行详细验证。

### 覆盖率指标

当前测试覆盖率统计：

| 模块 | Region 覆盖率 | 行覆盖率 | 函数覆盖率 |
|------|--------------|---------|-----------|
| collection.rs | 100.00% | 100.00% | 100.00% |
| condition.rs | 100.00% | 100.00% | 100.00% |
| error.rs | 100.00% | 100.00% | 100.00% |
| numeric.rs | 100.00% | 100.00% | 100.00% |
| **option.rs** | **76.19%** | **84.09%** | **100.00%** |
| string.rs | 100.00% | 100.00% | 100.00% |
| data_type.rs | 100.00% | 100.00% | 100.00% |
| pair.rs | 100.00% | 100.00% | 100.00% |
| triple.rs | 100.00% | 100.00% | 100.00% |
| **总计** | **98.38%** | **98.99%** | **100.00%** |

### 理解覆盖率指标

#### 为什么 option.rs 的 Region 覆盖率不是 100%？

`option.rs` 模块显示 76.19% 的 region 覆盖率，但实际上**可执行代码覆盖率为 100%**。这是由于 LLVM 覆盖率插桩的已知特性：

**LLVM Region 覆盖率包含不可执行代码**：
- Trait 定义和签名
- 类型参数声明
- `where` 子句约束
- 泛型边界
- 声明之间的空行

这些声明性元素被 LLVM 分配了 region ID，但它们**不是可执行代码**。`option.rs` 中的实际实现代码具有 **100% 覆盖率**（通过检查详细报告验证 - 没有表示未覆盖代码的 `^` 标记）。

**示例**：
```rust
pub trait OptionArgument<T> {              // ← 被计为一个 region
    fn require_non_null(                   // ← 被计为一个 region
        self,                              // ← 被计为一个 region
        name: &str                         // ← 被计为一个 region
    ) -> ArgumentResult<T>;                // ← 被计为一个 region

    fn require_non_null_and<F>(            // ← 被计为一个 region
        self, name: &str,
        predicate: F,
        error_msg: &str
    ) -> ArgumentResult<T>
    where                                  // ← 被计为一个 region
        F: FnOnce(&T) -> bool;             // ← 被计为一个 region
}
```

**发生原因**：
- `option.rs` 有约 88 行 trait 定义，包含复杂的泛型约束
- 简单模块如 `error.rs` 的声明性 regions 较少
- 这是 LLVM 覆盖率的已知限制（GitHub Issue [rust#79417](https://github.com/rust-lang/rust/issues/79417)）
- 使用 LLVM 覆盖率的 C++ 项目在处理模板时也面临类似问题

**重要指标**：
- ✅ **函数覆盖率：100%** - 所有函数都经过测试
- ✅ **行覆盖率：84.09%** - 实际代码覆盖率很高
- ✅ **可执行代码：100%** - 所有实现逻辑都经过测试
- ⚠️ **Region 覆盖率：76.19%** - 包含了不可执行的声明

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 为特定模块生成详细报告
cargo llvm-cov test --text | grep -A 50 "option.rs"
```

### 覆盖率工具的局限性

覆盖率统计由 `cargo-llvm-cov` 生成。解读结果时请注意：

1. **关注函数和行覆盖率** - 这些是最有意义的指标
2. **Region 覆盖率可能较低** - 特别是对于包含大量 trait 定义的模块
3. **通过详细报告验证** - 检查未覆盖区域中的 `^` 标记
4. **已知的 LLVM 问题** - 不可执行的声明也被计为 regions

更多详情请参见：
- [LLVM 覆盖率文档](https://doc.rust-lang.org/rustc/instrument-coverage.html)
- [Rust Issue #79417](https://github.com/rust-lang/rust/issues/79417) - Doctest 覆盖率映射
- 项目覆盖率报告位于 `target/llvm-cov/html/`

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

在贡献测试时，请注意对于包含大量 trait 定义的模块，达到 100% region 覆盖率并不总是可行的。
应专注于确保所有可执行代码路径都经过测试。

## 作者

**胡海星** - *Qubit Co. Ltd.*

---

项目仓库：[github.com/qubit-ltd/rs-common](https://github.com/qubit-ltd/rs-common)。

# Tuple Module

元组模块，提供带命名字段的泛型元组结构，提升代码可读性。

## 概述

本模块提供了两个带命名字段的泛型结构：

- **`Pair<F, S>`**: 用于存储两个值的泛型对结构
- **`Triple<F, S, T>`**: 用于存储三个值的泛型三元组结构

这些结构的成员变量都是 `pub` 公开的，可以直接访问和修改。

## 特性

### Pair

`Pair<F, S>` 是一个持有两个值的泛型结构，具有以下特性：

- **公共字段**: `first` 和 `second` 字段都是 `pub`，可以直接访问
- **灵活创建**: 可以使用 `new()` 方法或结构体字面量创建
- **类型转换**: 支持与元组 `(F, S)` 相互转换
- **函数式操作**: 提供 `map_first`、`map_second` 和 `swap` 方法
- **标准 trait**: 实现了 `Debug`、`Clone`、`Copy`、`PartialEq`、`Eq`、`Hash`、`Default` 和 `Display`

#### 示例

```rust
use qubit_common::Pair;

// 使用 new 方法创建
let pair1 = Pair::new("key", "value");

// 使用结构体字面量创建（公共字段）
let mut pair2 = Pair {
    first: 1,
    second: 2,
};

// 直接访问和修改字段
pair2.first = 10;
pair2.second = 20;

// 函数式操作
let mapped = pair1.map_first(|k| k.to_uppercase());
let swapped = pair2.swap();

// 与元组转换
let tuple: (i32, i32) = pair2.into();
let pair3: Pair<i32, i32> = (1, 2).into();
```

### Triple

`Triple<F, S, T>` 是一个持有三个值的泛型结构，具有以下特性：

- **公共字段**: `first`、`second` 和 `third` 字段都是 `pub`，可以直接访问
- **灵活创建**: 可以使用 `new()` 方法或结构体字面量创建
- **类型转换**: 支持与元组 `(F, S, T)` 相互转换
- **函数式操作**: 提供 `map_first`、`map_second` 和 `map_third` 方法
- **标准 trait**: 实现了 `Debug`、`Clone`、`Copy`、`PartialEq`、`Eq`、`Hash`、`Default` 和 `Display`

#### 示例

```rust
use qubit_common::Triple;

// 使用 new 方法创建
let triple1 = Triple::new("name", 42, true);

// 使用结构体字面量创建（公共字段）
let mut triple2 = Triple {
    first: 1,
    second: 2,
    third: 3,
};

// 直接访问和修改字段
triple2.first = 10;
triple2.second = 20;
triple2.third = 30;

// 函数式操作
let mapped = triple1
    .map_first(|s| s.to_uppercase())
    .map_second(|n| n * 2)
    .map_third(|b| !b);

// 与元组转换
let tuple: (i32, i32, i32) = triple2.into();
let triple3: Triple<i32, i32, i32> = (1, 2, 3).into();
```

## 使用场景

### Pair 的典型用例

1. **键值对**: 存储配置项或映射关系
2. **坐标**: 表示二维坐标 `(x, y)`
3. **范围**: 表示数值范围 `(min, max)`
4. **结果对**: 返回两个相关的值

### Triple 的典型用例

1. **记录**: 存储包含三个字段的简单记录
2. **坐标**: 表示三维坐标 `(x, y, z)`
3. **颜色**: 表示 RGB 颜色值
4. **版本**: 表示版本号 `(major, minor, patch)`

## 测试

所有测试都位于 `tests/util/tuple/` 目录下：

- `pair_tests.rs`: Pair 结构的单元测试
- `triple_tests.rs`: Triple 结构的单元测试

运行测试：

```bash
# 运行所有元组测试
cargo test --test util_tests

# 运行文档测试
cargo test --doc tuple

# 运行示例
cargo run --example util_demo
```

## API 参考

详细的 API 文档请参考：

- `pair.rs`: Pair 结构的完整实现
- `triple.rs`: Triple 结构的完整实现

## 作者

Haixing Hu

## 版权

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.


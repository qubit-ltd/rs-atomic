# Qubit Atomic 设计说明

本文记录 `qubit-atomic` 的设计决策，不作为 API 参考。具体方法列表、示例和签名以
rustdoc 与 `README.zh_CN.md` 为准；本文只保留语义边界和设计理由。

## 目标

`qubit-atomic` 是 Rust 原子类型之上的易用封装：

- 为常见场景隐藏显式内存序；
- 提供接近 JDK atomic 的高层操作；
- 通过有序 helper 和 `inner()` 保留高级出口；
- 让返回值语义清晰、稳定、可预测；
- 当值表示并发状态而不是纯指标时，避免静默 wrap。

本 crate 优先保持公共 API 紧凑，不追求为每个方法提供所有 ordering 变体。需要完全
自定义内存序时，调用方可以使用 `inner()`。

## 公共类型模型

公共 API 分为四层：

| 类型 | 目的 |
| --- | --- |
| `Atomic<T>` | 支持的基础值类型的泛型入口。 |
| `atomic::primitive::*` | 提供 `const fn new`，用于静态初始化。 |
| `AtomicRef<T>` | 原子替换 `Arc<T>`，并支持指针身份 CAS。 |
| `AtomicCount` / `AtomicSignedCount` | 用于状态计数和差值的 checked counter。 |
| `ArcAtomic*` | 对应 atomic container 的共享所有权封装。 |

基础值默认使用 `Atomic<T>`。具体 primitive wrapper 放在 `atomic::primitive` 下，
这样既保留静态初始化能力，也避免根 API 过宽。

## 返回值语义

命名遵循一组固定规则：

| 模式 | 含义 |
| --- | --- |
| `fetch_*` | 返回成功操作前观察到的旧值。 |
| `*_and_get` | 返回成功提交后的新值。 |
| `try_*` | 用户回调拒绝当前值时返回 `None`。 |
| `compare_set*` | 返回 `Result<(), actual>`，显式表达成功失败。 |
| `compare_and_exchange` | 返回观察到的值，保留 JDK 风格的便利形态。 |
| `compare_and_exchange_weak` | 返回 `Result<observed, actual>`，让 weak CAS 成功显式可见。 |

weak CAS 允许虚假失败。因此 `compare_and_exchange_weak` 不应返回裸值：返回值等于
`current` 并不能证明写入成功。`Result` 形状既保留观察值，又让成功和失败无歧义。

函数式更新闭包使用 `FnMut`。CAS loop 在竞争下可能多次调用闭包，调用方不能依赖回调
只执行一次。

## 内存序策略

默认内存序策略保持简单：

| 操作类别 | 默认内存序 | 理由 |
| --- | --- | --- |
| `load` | `Acquire` | 当 load 观察到 release sequence 时，让后续操作看到该 release 之前发布的数据；这不保证读到全局最新值。 |
| `store` | `Release` | 向 acquire 读取方发布之前的写入。 |
| `swap` 与 CAS | 成功 `AcqRel`，失败 `Acquire` | 标准读-改-写同步语义。 |
| 整数 `fetch_add/sub/inc/dec` | `Relaxed` | 常用于纯指标和计数。 |
| 整数有序 helper | 调用方指定 | 计数值同时作为同步信号时使用。 |
| CAS-loop 算术和更新 | 成功 `AcqRel`，失败 `Acquire` | 在重试循环中保留同步语义。 |
| 位运算和 max/min | `AcqRel` | 常用于标志、阈值或共享状态。 |
| `AtomicCount` 操作 | 成功 `AcqRel`，失败 `Acquire` | 计数被视为并发状态信号。 |

这是易用性默认值，不表示所有工作负载都必须使用这些内存序。性能敏感场景可以使用有序
整数 helper 或 `inner()`。

## 整数语义

基础整数操作遵循 Rust atomic integer 行为：

- `fetch_add`、`fetch_sub`、`fetch_inc`、`fetch_dec` 在溢出或下溢时 wrap；
- `fetch_mul`、`fetch_div`、`fetch_accumulate`、`accumulate_and_get` 按操作本身的
  wrapping 语义执行；
- 除零会 panic，符合 Rust 整数除法行为。

如果 wrap 是业务错误，应使用 `AtomicCount` 或 `AtomicSignedCount`。这些类型表示
状态型计数，并返回新值，便于处理状态迁移。

## 浮点语义

浮点 atomic 使用整数 atomic 存储原始 bit：

- `Atomic<f32>` 将 `f32::to_bits()` 存入 `AtomicU32`；
- `Atomic<f64>` 将 `f64::to_bits()` 存入 `AtomicU64`；
- CAS 比较原始 bit pattern，而不是 `PartialEq`。

因此 `0.0` 与 `-0.0` 不会 CAS 匹配，NaN payload 也必须完全一致。需要显式成功标志时，
使用 `compare_set` 或 `compare_set_weak`。

浮点算术通过 CAS loop 实现。它提供便利性，但不替代高竞争场景下的数值稳定累加方案。

## 性能与可移植性边界

性能主张应保持窄范围。基础类型 wrapper 使用 `#[repr(transparent)]`，简单 inline 转发
方法旨在优化为对应的后端操作。CAS-loop 算术、checked counter 更新与引用操作仍然保留
其固有的重试、引用计数、回收或分配成本。

`i128` 和 `u128` 特例使用 `portable-atomic`。是否原生 lock-free 取决于目标平台；目标
缺少合适原子指令时，其 fallback 可能使用锁。调用方不能从 crate API 推导出跨平台的
lock-free 保证。

基准应使用 Criterion，把代表性的 wrapper 操作与语义等价的 `std` 或 `arc-swap` 基线
比较。测量结果只对具体目标与工作负载构成证据，不是可移植的零开销保证。

## 引用语义

`AtomicRef<T>` 基于 `arc_swap::ArcSwap<T>`，存储 `Arc<T>`。CAS 比较的是指针身份，
不是 `T: Eq` 的值相等。
它只暴露强指针 CAS。weak 别名在 `arc_swap` 后端上没有独立语义，反而会让调用方以为
这里存在 weak CAS 的重试规则。

`AtomicRef<T>` 刻意不实现 `Clone`。显式的 `fork()` 方法会创建一个新的 atomic
container，并用一次 acquire load 观察到的 `Arc<T>` 初始化。两个容器最初共享同一个
`T`，但后续原子操作彼此独立。与并发写入竞争时，`fork()` 可能观察到较旧的值，不保证
获得全局最新值。多个所有者需要共享同一 atomic container 时，应使用
`ArcAtomicRef<T>` 或 `Arc<AtomicRef<T>>`。

`load_guard()` 适合短生命周期读取，可在 fast path 避免克隆 `Arc`。需要可移动、可保存
的 `Arc<T>` 时使用 `load()`。

## API 扩展原则

只有满足以下条件之一时才新增公共方法：

- 澄清重要返回值语义；
- 防止常见误用；
- 与 crate 中已有操作族保持一致；
- 暴露现有方法无法正确表达的必要能力。

不要因为可以实现就增加 `add_fetch`、`update_fetch` 这类别名。返回新值的操作优先使用
`*_and_get`，因为它可读，并且与 JDK 风格一致。

## 文档分工

文档分工如下：

- rustdoc：权威 API 签名和可运行示例；
- README：用户视角概览和常见工作流；
- 本设计说明：长期设计理由和语义边界。

避免在这里重复完整 API 表。如果一次代码变更需要大规模修改本文，说明本文又写得过细。

## 测试策略

测试应覆盖：

- 各操作族的返回值语义；
- checked 与 wrapping 行为差异；
- 浮点 CAS 的 raw-bit 行为；
- 引用 CAS 的指针身份行为；
- CAS loop 下回调可能重试的行为；
- 通过后端适配器在 Loom 下检查生产环境的 checked-counter CAS 核心；
- 与语义等价直接后端进行 Criterion 对照；
- rustdoc 不覆盖的 Markdown Rust 示例。

Markdown 示例测试应从 README 抽取 Rust code fence，生成临时 crate，并通过本地 path
dependency 编译当前 crate。这样可以防止文档漂移，而不把设计文档变成另一个 API 真相源。

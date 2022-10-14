# pallet-poe

编译：
```
cargo build --release
```

运行：

```
./target/release/node-template --dev
```

测试：
```
cargo test -p pallet-poe
```

交互：

1、打开https://polkadot.js.org/apps/  

2、连接本地网络，依次选择DEVELOPMENT，127.0.0.1:9944

3、找到“开发者”>“交易”>“poeModule”，选择相应的方法进行交易

4、找到“链状态”>“poeModule”>相应的存储项，可以查询存储值

## 补充性能测试

参考凯超老师的仓库：https://github.com/kaichaosun/play-substrate/

搜索benchmark分支下“性能测试”字样，就能查看到代码集成步骤，总共5步。

1、编译：
```
cargo build --release --features runtime-benchmarks
```

2、复制测试报告模板

由于我们当前的代码示例是基于`polkadot-v0.9.25`版本，所以在对应的版本中找到这个文件：https://github.com/paritytech/substrate/blob/polkadot-v0.9.25/.maintain/frame-weight-template.hbs，将其复制到当前项目中对应的目录下。

3、开始性能测试

执行：
```
./target/release/node-template benchmark pallet \
--chain dev \
--execution wasm \
--wasm-execution compiled \
--pallet pallet_poe --extrinsic "*" \
--steps 20 --repeat 10 \
--output ./pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs
```

返回结果如：

```
Pallet: "pallet_poe", Extrinsic: "create_claim", Lowest values: [], Highest values: [], Steps: 20, Repeat: 10
Raw Storage Info
========
Storage: PoeModule Proofs (r:1 w:1)

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=    17.23
    + d    0.003
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    d   mean µs  sigma µs       %
    0     16.66     1.105    6.6%
   25     17.83     0.372    2.0%
   50     16.83     0.687    4.0%
   75      17.5     0.763    4.3%
  100        17         0    0.0%
  125        17         0    0.0%
  150     18.16     0.372    2.0%
  175        20     0.816    4.0%
  200     18.33     0.471    2.5%
  225     19.16     0.897    4.6%
  250     18.66     0.471    2.5%
  275        18         0    0.0%
  300     17.66     0.471    2.6%
  325        18         0    0.0%
  350     18.16     0.372    2.0%
  375        18         0    0.0%
  400     18.16     0.372    2.0%
  425     18.33     0.471    2.5%
  450     19.83     0.687    3.4%
  475     18.33     0.471    2.5%
  500     19.66     0.471    2.3%

Quality and confidence:
param     error
d             0

Model:
Time ~=    17.27
    + d    0.004
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
```

4、集成weights.rs

通过执行第4步的命令，在`./pallets/poe/src/`目录会输出一个`weights.rs`文件

搜索“性能测试”字样，查看各文件的集成修改情况


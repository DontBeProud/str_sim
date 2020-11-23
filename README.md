# str_sim
rust

## Introduction
```
计算字符串之间的相似度

calculate the similarity between strings

currently contains the following algorithms : 【levenshtein、jaro、jaro_winkler】
```

## Installation
```toml
[dependencies]
str_sim = "0.1.2"
```

## Examples
```
use str_sim::{levenshtein_distance, sim_jaro_winkler, sim_jaro};
fn main() {
        assert_eq!(levenshtein_distance("dontbeproud", "dontbepride"), 3);
        assert_eq!(jaro::sim_jaro("dontbeproud", "dontbepride"), 0.8787878787878789);
        assert_eq!(jaro_winkler::sim_jaro_winkler("dontbeproud", "dontbepride", 0.05), 0.9272727272727274);
}
```

## Tips
```
另外，另一个用于分析账号之间相似度的项目正在开发中
In addition, another project for analyzing the similarity between accounts is under development
<https://github.com/DontBeProud/calculate_account_names_similarity_rs>
<https://gitee.com/DontBeProud/calculate_account_names_similarity_rs>
```



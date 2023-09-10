### 値オブジェクト

システム固有の値を表現するために定義されたオブジェクト。

> **値の特徴**

- 不変である

```
let full_name = "hamada tomoki";
name.change_to("hamada taro"); // 値が変更される不自然

let full_name = FullName::new("hamada", "tomoki);
full_name.change_to("hamda");
```

setter を用意したクラスのインスタンスを生成したことで、自身のあずかり知らぬところでいつの間にか状態が変更され、意図せぬバグを引き起こしてしまう可能があります。このような状態の変更を起因とするバグを防ぐために、最も効果的で強力な防衛策は最初から「不変」として扱うことです。いつの間にか変更されることが問題であるなら、そもそも変更できないようにしてしまえばいいのです。

> オブジェクトの不変にすることによるデメリットも、もちろん存在します。オブジェクトの値を一部変更したい場合、交換のため新たなインスタンスを生成する必要があるということです。パフォーマンスの観点で深刻な事態となる場合には、可変を許容する戦略をとることもありますが、いったんは不変として扱うのが安全でしょう。

- 交換が可能である

```
let mut full_name = "hamada tomoki";
full_name = "hoge";

let mut full_name = FullName::new("hamada", "tomoki");
full_name = FullName::new("hoge", "huga");
```

値は不変ですが、交換することは可能です。「不変」の値を持つ値はそれ自体を変更できません。値の変更は代入操作によって交換することで表現されます。

- 等価性によって比較される

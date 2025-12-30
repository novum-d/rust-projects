
実装するオプション
- w 単語数
- c バイト数
- l 改行数


実行例

仕様

- 引数に複数のファイルを与えると、出力の最後に数・単語数・バイト数の合計値が表示される
    ```shell
    $ wc -lwc tests/inputs/* 
      4  29 177 tests/inputs/atlamal.txt
      0   0   0 tests/inputs/empty.txt
      1   9  48 tests/inputs/fox.txt
      5  38 225 合計
    ```
- 存在しないファイルを指定すると、ファイルの処理中に標準エラーに警告が表示される
    ```shell
    $ wc -lwc tests/inputs/atlamal.txt blargh tests/inputs/fox.txt 
      4  29 177 tests/inputs/atlamal.txt
      wc: blargh: No such file or directory
      1   9  48 tests/inputs/fox.txt
      5  38 225 合計
    ```
 
- 標準エラー出力をファイルにリダイレクトすることでエラーメッセージがファイルに書き込まれる 
    ```shell
    $ wc -lwc tests/inputs/atlamal.txt blargh tests/inputs/fox.txt 2>err
      4  29 177 tests/inputs/atlamal.txt
      1   9  48 tests/inputs/fox.txt
      5  38 225 合計
    $ cat err
    wc: blargh: No such file or directory
    ```



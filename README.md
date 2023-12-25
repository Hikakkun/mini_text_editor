# mini text editor(rust)
* Youtube で見たC言語で最小限のテキストエディタを練習がてらRustで実装する[^1]

## プログラム[^2]の基本的な動作
1. コマンドライン引数から編集したいファイルのパスを取得
2. ファイルを読み取り専用モードで開く
3. 読み込んだファイルをすべてコマンドラインに出力
4. 読み込んだファイルの任意の行で変更
5. ファイルを書き込みモードで開き編集後の内容をファイルに書き戻す

## 実行方法
```bash
/mini_text_editor> cargo run <edit_file_path>
```

## 使用方法
* 変更したいファイルのパスをコマンドライン引数として入力
* プログラムに従って以下を入力
    * 変更したい行番号
    * 変更後の文字列
* 書き込み完了後は cat コマンドなどで確認して下しあ
```bash
/mini_text_editor> cargo run <edit_file_path>
edit_file_path: <edit_file_path>
Contents
00|zero
01|one
02|two 
03|three
04|four
input edit line number>00
line00>00zero00
Data has been written to <edit_file_path> successfully.
```

## 引用元
[^1]: [Making Minimalist Text Editor in C on Linux](https://www.youtube.com/watch?v=gnvDPCXktWQ)
[^2]: [nir9/editor.c](https://gist.github.com/nir9/6398b692acbcd44a66141004b2882009)

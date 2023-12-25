# mini text editor(rust)
* Youtube で見たC言語で最小限のテキストエディタを練習がてらRustで実装する[^1]

## プログラム[^2]の基本的な動作
1. コマンドライン引数から編集したいファイルのパスを取得
2. ファイルを読み取り専用モードで開く
3. 読み込んだファイルをすべてコマンドラインに出力
4. 読み込んだファイルの任意の行で変更
5. ファイルを書き込みモードで開き編集語の内容をファイルに書き戻す

## 引用元
[^1]: [Making Minimalist Text Editor in C on Linux](https://www.youtube.com/watch?v=gnvDPCXktWQ)
[^2]: [nir9/editor.c](https://gist.github.com/nir9/6398b692acbcd44a66141004b2882009)
```c:editor.c
#include <stdio.h>
#include <string.h>

void edit_line(char* buffer, int current_line) {
    for (int i = 0; i < current_line; i++) {
        buffer = strchr(buffer, '\n') + 1;
    }

    char* line_end = strchr(buffer, '\n');
    char saved[1024] = { 0 };
    strcpy(saved, line_end);
    scanf("%s", buffer);
    strcpy(buffer + strlen(buffer), saved);
}

void main(int argc, char** argv) {
    FILE* f = fopen(argv[1], "r");
    char buffer[1024] = {0};
    fread(buffer, 1024, 1, f);
    fclose(f);
    printf("Contents:\n%s\n", buffer);
    int current_line = 0;
    scanf("%d", &current_line);
    edit_line(buffer, current_line);
    f = fopen(argv[1], "w");
    fwrite(buffer, strlen(buffer), 1, f);
    fclose(f);
}
```


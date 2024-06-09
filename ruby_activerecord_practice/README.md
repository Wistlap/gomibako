# Ruby ActiveRecord でデータベース操作

ActiveRecord を使った基本的なデータベース操作の走り書き

## ファイル構成
| ファイル名 | 説明 |
|:-------------|:----------|
| main.rb | 実際の操作を記述したプログラム |
| config.yaml | ActiveRecord のコンフィグファイル |
| practice.sqlite3 | データベース |

## データベース構成
| テーブル名 | 説明 |
|:-------------|:----------|
| projects | 何かしらのプロジェクト名 |
| tags | 何かしらのタグ |
| types | 何かしらのタイプ |

各テーブルのカラムは `id` と `content` から成る

## 使い方
好きなように `main.rb` を弄って以下を実行
```
ruby main.rb
```

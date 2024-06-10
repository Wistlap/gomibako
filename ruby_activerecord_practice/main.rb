require 'yaml'
require 'active_record'

# DB 設定ファイル読込み
config = YAML.load_file('./config.yml')
ActiveRecord::Base.establish_connection(config['sqlite3']['development'])

# プロジェクトテーブルのクラスを宣言
class Projects < ActiveRecord::Base
    self.table_name = 'projects'
    self.primary_key = :id
end

# タイプテーブルのクラスを宣言
class Types < ActiveRecord::Base
    self.table_name = 'types'
    self.primary_key = :id
end

# プロジェクトテーブルの登録データを全件取得，反転，1件ずつ表示
project_list = Projects.all #project_list のクラスは ActiveRecord::Relation
project_list_reverse = project_list.reverse #project_list_reverse のクラスは Array
project_list_reverse.each do |project|
    p project
end

# タイプテーブルの登録データを全件取得，反転，1件ずつ表示
type_list = Types.all.to_a
type_list.each do |type|
    p type
end

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

# 単一のプロジェクトのクラスを宣言
class Project
    def initialize
        @id = 0
        @content = ""
    end

    attr_accessor :id
    attr_accessor :content
end

# タイプテーブルのクラスを宣言
class Types < ActiveRecord::Base
    self.table_name = 'types'
    self.primary_key = :id
end

# 単一のタイプのクラスを宣言
class Type
    def initialize
        @id
        @content
    end

    attr_accessor :id
    attr_accessor :content
end

# プロジェクトテーブルの登録データを全件取得，反転，1件ずつ表示
project_list = Projects.all #project_list のクラスは ActiveRecord::Relation
project_list_reverse = project_list.reverse #project_list_reverse のクラスは Array
puts "登録プロジェクトを逆順に全件表示"
project_list_reverse.each do |project|
    p project
end

# 特定のプロジェクトを取得
project = Project.new
res = Projects.find(5)
project.id, project.content = res.id, res.content
puts "特定のプロジェクトを表示"
p project

# タイプテーブルの登録データを全件取得，反転，1件ずつ表示
type_list = Types.all.to_a
puts "登録タイプを全件表示"
type_list.each do |type|
    p type
end

# タイプを追加
type = Type.new
type.content = "その他"
types = Types.new
types.id, types.content = type.id, type.content
types.save
type_list = Types.all.to_a
puts "タイプを追加，全件表示"
type_list.each do |type|
    p type
end


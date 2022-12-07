use anyhow::*;


/// APIの抽象化レイヤー
pub trait Api {
    /// 抜け落ちを含む任意のウニ文字列から新しいウニを検出し，データベースに追加する
    async fn from_dust(&self, s: Vec<String>) -> Result<Vec<String>>;

    /// すべてのウニを文字列として出力する
    async fn list_all(&self) -> Result<Vec<String>>;

    /// 文字数制限に収まる範囲でウニを文字列として出力する
    async fn list_short(&self) -> Result<Vec<String>>;

    /// 指定された位置にウニ文字列を追加する
    async fn add(&self, content: String, pos: i32) -> Result<()>;

    /// 指定された位置のウニ文字列を削除する
    async fn remove(&self, pos: i32) -> Result<()>;

    /// 位置を2つ指定し，それらの位置を入れ替える
    async fn swap(&self, pos_1: i32, pos_2: i32) -> Result<()>;
}

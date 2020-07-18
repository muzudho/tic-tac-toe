//! Parses the command to make it easier to use.  
//! コマンドを解析して、使いやすくします。  
use crate::LogExt;
use casual_logger::Log;
use std::fmt;

/// Parses the command to make it easier to use.  
/// コマンドを解析して、使いやすくします。  
pub struct CommandLineSeek {
    /// The entire command line string.  
    /// コマンドライン全体の文字列です。  
    line: String,
    /// The length of the string.  
    /// 文字列の長さ。  
    len: usize,
    /// The reading position.  
    /// 読み取り位置です。  
    pub current: usize,
}
impl CommandLineSeek {
    /// Create a parser.  
    /// パーサーを作成します。  
    ///
    /// # Arguments
    ///
    /// * `line` - Specify a character string that does not include a line break.  
    ///             改行を含まない文字列を指定してください。  
    pub fn new(line: &str) -> Self {
        // Erase the trailing newline.
        // 末尾の改行を削除します。
        let line: String = match line.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!(Log::print_fatal(&format!(
                "(Err.38)  Failed to parse. / {}",
                e
            ))),
        };
        // character count.
        // 文字数。
        let len = line.chars().count();
        CommandLineSeek {
            line: line,
            len: len,
            current: 0,
        }
    }

    /// Does the character match from the beginning?  
    /// 文字は先頭から一致していますか？  
    ///
    /// # Arguments
    ///
    /// * `expected` - A string that starts from the beginning.  
    ///                 先頭から始まる文字列。  
    ///
    /// # Returns
    ///
    /// True if so.  
    /// そうであれば真です。  
    pub fn starts_with(&self, starting: &str) -> bool {
        let len2 = starting.len();
        len2 <= self.len && &self.line[self.current..len2] == starting
    }

    /// Advance the scanning position.  
    /// 読み取り位置を進めます。  
    ///
    /// # Arguments
    ///
    /// * `expected` - The string to skip.  
    ///                 読み飛ばす文字列。  
    pub fn go_next_to(&mut self, skip: &str) {
        self.current += skip.len();
    }

    /// The rest of the command line.  
    /// コマンドラインの残りの部分です。  
    ///
    /// # Returns
    ///
    /// The rest of the command line.  
    /// コマンドラインの残りの部分です。  
    pub fn rest(&self) -> Option<&str> {
        if self.current < self.line.len() {
            Some(&self.line[self.current..])
        } else {
            None
        }
    }
}
impl fmt::Debug for CommandLineSeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
        // Tips. It is convenient to make a table by enclosing it with vertical bars.  
        // Example: value=|apple|banana|cherry|  
        // テクニック。 '|' で囲んでテーブルを作成すると便利です。  
        // 例: value=|りんご|バナナ|さくらんぼ|  
        "line=|{}| len={} starts={}",
            self.line, self.len, self.current
        )
    }
}

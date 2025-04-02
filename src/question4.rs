// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
use std::fs::File;
use std::io::{self, Write, BufReader};
use std::path::Path;

pub fn count_file_stats(file: &File) -> io::Result<(usize, usize)> {
    let reader = BufReader::new(file);
    let mut char_count = 0;
    let mut line_count = 0;

    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        char_count += line.chars().count();
    }

    Ok((char_count, line_count))
}

pub fn format_stats(file_path: &str, char_count: usize, line_count: usize) -> String {
    format!(
        "文件 '{}' 的统计结果:\n字符数（不含换行符）: {}\n行数: {}\n",
        file_path, char_count, line_count
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_count_file_stats() -> io::Result<()> {
        // 创建内存中的测试文件
        let data = "Hello 世界\nLine 2\nLine 3";
        let cursor = Cursor::new(data);
        let file = File::create("test_file.txt")?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(data.as_bytes())?;
        writer.flush()?;

        // 重新打开用于读取
        let file = File::open("test_file.txt")?;
        let (chars, lines) = count_file_stats(&file)?;
        
        assert_eq!(lines, 3);
        assert_eq!(chars, 20); // "Hello 世界"=9 + "Line 2"=6 + "Line 3"=5
        
        // 清理
        std::fs::remove_file("test_file.txt")?;
        Ok(())
    }

    #[test]
    fn test_empty_file() -> io::Result<()> {
        let file = File::create("empty.txt")?;
        let (chars, lines) = count_file_stats(&file)?;
        assert_eq!(chars, 0);
        assert_eq!(lines, 0);
        std::fs::remove_file("empty.txt")?;
        Ok(())
    }

    #[test]
    fn test_format_stats() {
        let output = format_stats("test.txt", 125, 10);
        assert!(output.contains("字符数（不含换行符）: 125"));
        assert!(output.contains("行数: 10"));
        assert!(output.contains("文件 'test.txt'"));
    }

    #[test]
    fn test_unicode_chars() -> io::Result<()> {
        let data = "こんにちは\n你好";
        let file = File::create("unicode.txt")?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(data.as_bytes())?;
        writer.flush()?;

        let file = File::open("unicode.txt")?;
        let (chars, lines) = count_file_stats(&file)?;
        assert_eq!(lines, 2);
        assert_eq!(chars, 8); // こんにちは=5 + 你好=3
        
        std::fs::remove_file("unicode.txt")?;
        Ok(())
    }
}
use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 1. 按buffer大小读取文件
    // let mut f = File::open("foo.txt").await?;
    // let mut buffer = [0; 10];

    // // 由于 buffer 的长度限制，当次的 `read` 调用最多可以从文件中读取 10 个字节的数据
    // let n = f.read(&mut buffer[..]).await?;

    // println!("The bytes: {:?}", &buffer[..n]);
    // Ok(())

    // 2.写入文件
    // let mut file = File::create("foo.txt").await?;
    // let n = file.write(b"some bytes").await?;

    // println!("Wrote {} bytes", n);
    // Ok(())

    //3.reader => writer
    let mut reader: &[u8] = "hello world".as_bytes();
    let mut writer = File::create("foo.txt").await?;
    let n = io::copy(&mut reader, &mut writer).await?;
    println!("Wrote {} bytes", n);
    Ok(())
}

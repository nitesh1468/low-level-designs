#![allow(unused)]

trait DataSource {
    fn write_data(&mut self, data: &str);
    fn read_data(&self) -> String;
}

#[derive(Debug)]
struct FileDataSource {
    file_name: String,
    content: String,
}

impl FileDataSource {
    fn new(file_name: &str) -> Box<dyn DataSource> {
        Box::new(Self {
            file_name: file_name.to_owned(),
            content: String::new(),
        })
    }
}

impl DataSource for FileDataSource {
    fn write_data(&mut self, data: &str) {
        self.content = data.to_owned();
        println!("[File] saved to disk: {}", self.content);
    }
    fn read_data(&self) -> String {
        // read from file
        self.content.clone()
    }
}

struct EncryptionDecorator {
    wrapped: Box<dyn DataSource>,
}

impl EncryptionDecorator {
    fn new(source: Box<dyn DataSource>) -> Box<dyn DataSource> {
        Box::new(Self { wrapped: source })
    }
}

impl EncryptionDecorator {
    fn encrypt(&self, data: &str) -> String {
        format!("encrypted:{data}")
    }

    fn decrypt<'a>(&self, data: &'a str) -> String {
        (&data[10..]).to_owned()
    }
}

impl DataSource for EncryptionDecorator {
    fn write_data(&mut self, data: &str) {
        let encrypted = self.encrypt(data);
        self.wrapped.as_mut().write_data(&encrypted);
    }

    fn read_data(&self) -> String {
        let data = self.wrapped.read_data();
        self.decrypt(&data)
    }
}

struct CompressionDecorator {
    wrapped: Box<dyn DataSource>,
}

impl CompressionDecorator {
    fn new(source: Box<dyn DataSource>) -> Box<dyn DataSource> {
        Box::new(Self { wrapped: source })
    }
}

impl CompressionDecorator {
    fn compress(&self, data: &str) -> String {
        format!("compressed:{data}")
    }

    fn decompress<'a>(&self, data: &'a str) -> String {
        (&data[11..]).to_owned()
    }
}

impl DataSource for CompressionDecorator {
    fn write_data(&mut self, data: &str) {
        let compressed = self.compress(data);
        self.wrapped.as_mut().write_data(&compressed);
    }

    fn read_data(&self) -> String {
        let data = self.wrapped.read_data();
        self.decompress(&data)
    }
}

fn main() {
    let mut source = FileDataSource::new("data.txt");
    source = CompressionDecorator::new(source);
    source = EncryptionDecorator::new(source);

    println!("--- Writing Data ---");
    source.as_mut().write_data("sensitive info");
    println!("--- Reading Data ---");
    println!("Final Restored Output: {}", source.as_mut().read_data());
}

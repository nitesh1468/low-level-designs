#include <iostream>
#include <memory>
#include <string>
#include <utility>

// 1. Component Interface
class DataSource {
public:
    virtual ~DataSource() = default;
    virtual void writeData(const std::string& data) = 0;
    virtual std::string readData() = 0;
};

// 2. Concrete Component
class FileDataSource : public DataSource {
public:
    explicit FileDataSource(std::string filename) : filename(std::move(filename)) {}

    void writeData(const std::string& data) override {
        // Simulating writing to a file by storing it in a member variable
        fileContent = data; 
        std::cout << "[File] Saved to disk: " << fileContent << "\n";
    }

    std::string readData() override {
        // Simulating reading from a file
        return fileContent;
    }

private:
    std::string filename;
    std::string fileContent; // Mock file storage
};

// 3. Concrete Decorator A
class EncryptionDecorator : public DataSource {
public:
    explicit EncryptionDecorator(std::shared_ptr<DataSource> source) : wrapped(std::move(source)) {}

    void writeData(const std::string& data) override {
        auto encrypted = encrypt(data);
        wrapped->writeData(encrypted);
    }

    std::string readData() override {
        auto data = wrapped->readData();
        return decrypt(data);
    }

private:
    std::string encrypt(const std::string& data) { return "encrypted:" + data; }
    std::string decrypt(const std::string& data) { return data.substr(10); }

    std::shared_ptr<DataSource> wrapped;
};

// 4. Concrete Decorator B
class CompressionDecorator : public DataSource {
public:
    explicit CompressionDecorator(std::shared_ptr<DataSource> source) : wrapped(std::move(source)) {}

    void writeData(const std::string& data) override {
        auto compressed = compress(data);
        wrapped->writeData(compressed);
    }

    std::string readData() override {
        auto data = wrapped->readData();
        return decompress(data);
    }

private:
    std::string compress(const std::string& data) { return "compressed:" + data; }
    std::string decompress(const std::string& data) { return data.substr(11); }

    std::shared_ptr<DataSource> wrapped;
};

// 5. Execution Entry Point
int main() {
    // FIX: Stack decorators so that the reading order perfectly mirrors writing order.
    // Data flow for writing: Encryption -> Compression -> File
    std::shared_ptr<DataSource> source = std::make_shared<FileDataSource>("data.txt");
    source = std::make_shared<CompressionDecorator>(source);
    source = std::make_shared<EncryptionDecorator>(source);

    std::cout << "--- Writing Data ---\n";
    source->writeData("sensitive info");

    std::cout << "\n--- Reading Data ---\n";
    std::string result = source->readData();
    std::cout << "Final Restored Output: " << result << "\n";

    return 0;
}


#include <iostream>

// Adaptee (需要适配的类)
class OldPrinter
{
public:
    void printText(const std::string &text)
    {
        std::cout << "Old Printer: " << text << std::endl;
    }
};

// Target (目标接口)
class NewPrinter
{
public:
    virtual void printText(const std::string &text) = 0;
    virtual void printImage(const std::string &image) = 0;
};

// Adapter
class PrinterAdapter : public NewPrinter
{
private:
    OldPrinter oldPrinter;

public:
    void printText(const std::string &text) override
    {
        oldPrinter.printText(text);
    }

    void printImage(const std::string &image) override
    {
        std::cout << "Printing Image: " << image << std::endl;
    }
};

int main()
{
    NewPrinter *printer = new PrinterAdapter();
    printer->printText("Hello, World!");
    printer->printImage("SampleImage.jpg");

    delete printer;
    return 0;
}

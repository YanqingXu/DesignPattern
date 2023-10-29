#include <iostream>
#include <vector>

// 抽象处理者类
class Handler {
public:
    virtual ~Handler() {}
    void SetSuccessor(Handler* successor) {
        successor_ = successor;
    }
    virtual void HandleRequest(int request) = 0;

protected:
    Handler* successor_;
};

// 具体处理者类1
class ConcreteHandler1 : public Handler {
public:
    virtual void HandleRequest(int request) override {
        if (request < 10) {
            std::cout << "ConcreteHandler1 handled request " << request << std::endl;
        } else if (successor_ != nullptr) {
            successor_->HandleRequest(request);
        }
    }
};

// 具体处理者类2
class ConcreteHandler2 : public Handler {
public:
    virtual void HandleRequest(int request) override {
        if (request >= 10 && request < 20) {
            std::cout << "ConcreteHandler2 handled request " << request << std::endl;
        } else if (successor_ != nullptr) {
            successor_->HandleRequest(request);
        }
    }
};

// 客户端代码
int main() {
    ConcreteHandler1 handler1;
    ConcreteHandler2 handler2;

    handler1.SetSuccessor(&handler2);

    std::vector<int> requests = {2, 5, 14, 22, 18, 3, 27, 20};

    for (int request : requests) {
        handler1.HandleRequest(request);
    }

    return 0;
}

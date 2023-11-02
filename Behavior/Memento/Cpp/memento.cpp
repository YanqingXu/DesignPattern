#include <iostream>
#include <vector>
#include <memory>

// Memento类存储Originator对象的状态
class Memento
{
private:
    std::string state_;

    // 通常只允许Originator访问这个状态
    friend class Originator;

public:
    Memento(const std::string &state) : state_(state) {}

    std::string GetState() const
    {
        return state_;
    }
};

// Originator类可以创建一个备忘录，并可以使用备忘录恢复其状态
class Originator
{
private:
    std::string state_;

public:
    void SetState(const std::string &state)
    {
        std::cout << "Originator: Setting state to " << state << std::endl;
        this->state_ = state;
    }

    std::unique_ptr<Memento> Save()
    {
        return std::make_unique<Memento>(state_);
    }

    void Restore(const Memento &memento)
    {
        state_ = memento.GetState();
        std::cout << "Originator: State after restoring from memento: " << state_ << std::endl;
    }
};

// Caretaker负责保存备忘录，但是不能修改备忘录的内容
class Caretaker
{
private:
    std::vector<std::unique_ptr<Memento>> mementos_;
    Originator &originator_;

public:
    Caretaker(Originator &originator) : originator_(originator) {}

    void Backup()
    {
        std::cout << "Caretaker: Saving Originator's state..." << std::endl;
        mementos_.push_back(originator_.Save());
    }

    void Undo()
    {
        if (mementos_.empty())
        {
            std::cout << "Caretaker: No mementos to undo." << std::endl;
            return;
        }
        auto memento = std::move(mementos_.back());
        mementos_.pop_back();
        std::cout << "Caretaker: Restoring state from memento..." << std::endl;
        originator_.Restore(*memento);
    }
};

int main()
{
    Originator originator;
    Caretaker caretaker(originator);

    originator.SetState("State #1");
    caretaker.Backup();

    originator.SetState("State #2");
    caretaker.Backup();

    originator.SetState("State #3");
    caretaker.Undo();

    originator.SetState("State #4");
    caretaker.Undo();

    return 0;
}

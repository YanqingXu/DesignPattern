#include <iostream>

class Command
{
public:
    virtual ~Command() {}
    virtual void execute() = 0;
};

class Receiver
{
public:
    void action()
    {
        std::cout << "Receiver::action()" << std::endl;
    }
};

class ConcreteCommand : public Command
{
public:
    ConcreteCommand(Receiver *receiver) : _receiver(receiver) {}

    void execute() override
    {
        _receiver->action();
    }

private:
    Receiver *_receiver;
};

class Invoker
{
public:
    void setCommand(Command *command)
    {
        _command = command;
    }
    void executeCommand()
    {
        _command->execute();
    }

private:
    Command *_command;
};

int main()
{
    Receiver *receiver = new Receiver();
    Command *command = new ConcreteCommand(receiver);
    Invoker *invoker = new Invoker();
    invoker->setCommand(command);
    invoker->executeCommand();

    delete receiver;
    delete command;
    delete invoker;
    return 0;
}

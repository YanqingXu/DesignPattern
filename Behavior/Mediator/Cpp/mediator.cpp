#include <iostream>
#include <string>

class Colleague;

// Mediator interface
class Mediator
{
public:
    virtual void Send(const std::string &message, Colleague *colleague) = 0;
};

// Abstract colleague class
class Colleague
{
protected:
    Mediator *mediator;

public:
    Colleague(Mediator *med) : mediator(med) {}

    virtual void Send(const std::string &message)
    {
        mediator->Send(message, this);
    }

    virtual void Notify(const std::string &message) = 0;
};

// Concrete mediator class
class ConcreteMediator : public Mediator
{
private:
    Colleague *colleague1;
    Colleague *colleague2;

public:
    void SetColleague1(Colleague *col1)
    {
        colleague1 = col1;
    }

    void SetColleague2(Colleague *col2)
    {
        colleague2 = col2;
    }

    void Send(const std::string &message, Colleague *colleague) override
    {
        if (colleague == colleague1)
        {
            colleague2->Notify(message);
        }
        else
        {
            colleague1->Notify(message);
        }
    }
};

// Concrete colleague classes
class ConcreteColleague1 : public Colleague
{
public:
    ConcreteColleague1(Mediator *med) : Colleague(med) {}

    void Notify(const std::string &message) override
    {
        std::cout << "Colleague1 gets message: " << message << std::endl;
    }
};

class ConcreteColleague2 : public Colleague
{
public:
    ConcreteColleague2(Mediator *med) : Colleague(med) {}

    void Notify(const std::string &message) override
    {
        std::cout << "Colleague2 gets message: " << message << std::endl;
    }
};

// Example usage
int main()
{
    ConcreteMediator mediator;

    ConcreteColleague1 colleague1(&mediator);
    ConcreteColleague2 colleague2(&mediator);

    mediator.SetColleague1(&colleague1);
    mediator.SetColleague2(&colleague2);

    colleague1.Send("Hello, World!"); // Colleague1 sends a message
    colleague2.Send("Hi there!");     // Colleague2 sends a message

    return 0;
}

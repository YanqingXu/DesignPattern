#include <iostream>

class Expression
{
public:
    virtual int interpret() = 0;
    virtual ~Expression() {}
};

class NumberExpression : public Expression
{
    int number;

public:
    NumberExpression(int num) : number(num) {}

    int interpret() override
    {
        return number;
    }
};

class AddExpression : public Expression
{
    Expression *left;
    Expression *right;

public:
    AddExpression(Expression *l, Expression *r) : left(l), right(r) {}

    int interpret() override
    {
        return left->interpret() + right->interpret();
    }

    ~AddExpression()
    {
        delete left;
        delete right;
    }
};

class SubtractExpression : public Expression
{
    Expression *left;
    Expression *right;

public:
    SubtractExpression(Expression *l, Expression *r) : left(l), right(r) {}

    int interpret() override
    {
        return left->interpret() - right->interpret();
    }

    ~SubtractExpression()
    {
        delete left;
        delete right;
    }
};

int main()
{
    Expression *left = new NumberExpression(1);
    Expression *right = new NumberExpression(2);
    Expression *addition = new AddExpression(left, right);

    std::cout << "1 + 2 = " << addition->interpret() << std::endl;

    delete addition; // 会自动删除左右表达式

    return 0;
}

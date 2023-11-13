#include <iostream>
#include <vector>

// 元素接口
class Visitor; // 前置声明

class Element
{
public:
    virtual ~Element() = default;
    virtual void Accept(Visitor &visitor) = 0;
};

// 角色
class Character : public Element
{
public:
    void Accept(Visitor &visitor) override;

    void InteractWithCharacter()
    {
        std::cout << "Character interaction." << std::endl;
    }
};

// 敌人
class Enemy : public Element
{
public:
    void Accept(Visitor &visitor) override;

    void InteractWithEnemy()
    {
        std::cout << "Enemy interaction." << std::endl;
    }
};

// 道具
class Item : public Element
{
public:
    void Accept(Visitor &visitor) override;

    void InteractWithItem()
    {
        std::cout << "Item interaction." << std::endl;
    }
};

// 访问者接口
class Visitor
{
public:
    virtual ~Visitor() = default;
    virtual void VisitCharacter(Character &character) = 0;
    virtual void VisitEnemy(Enemy &enemy) = 0;
    virtual void VisitItem(Item &item) = 0;
};

// 交互访问者
class InteractionVisitor : public Visitor
{
public:
    void VisitCharacter(Character &character) override
    {
        character.InteractWithCharacter();
    }

    void VisitEnemy(Enemy &enemy) override
    {
        enemy.InteractWithEnemy();
    }

    void VisitItem(Item &item) override
    {
        item.InteractWithItem();
    }
};

void Character::Accept(Visitor &visitor)
{
    visitor.VisitCharacter(*this);
}

void Enemy::Accept(Visitor &visitor)
{
    visitor.VisitEnemy(*this);
}

void Item::Accept(Visitor &visitor)
{
    visitor.VisitItem(*this);
}

int main()
{
    std::vector<Element *> elements;
    elements.push_back(new Character());
    elements.push_back(new Enemy());
    elements.push_back(new Item());

    InteractionVisitor visitor;

    for (auto &element : elements)
    {
        element->Accept(visitor);
    }

    // 清理
    for (auto &element : elements)
    {
        delete element;
    }

    return 0;
}

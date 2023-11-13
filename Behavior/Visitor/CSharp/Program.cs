using System;

// 元素接口
public interface IElement
{
    void Accept(IVisitor visitor);
}

// 角色
public class Character : IElement
{
    public string Name { get; set; } = "Hero";

    public void Accept(IVisitor visitor)
    {
        visitor.VisitCharacter(this);
    }
}

// 敌人
public class Enemy : IElement
{
    public int Health { get; set; } = 100;

    public void Accept(IVisitor visitor)
    {
        visitor.VisitEnemy(this);
    }
}

// 道具
public class Item : IElement
{
    public string ItemName { get; set; } = "Sword";

    public void Accept(IVisitor visitor)
    {
        visitor.VisitItem(this);
    }
}

// 访问者接口
public interface IVisitor
{
    void VisitCharacter(Character character);
    void VisitEnemy(Enemy enemy);
    void VisitItem(Item item);
}

// 序列化访问者
public class SerializeVisitor : IVisitor
{
    public void VisitCharacter(Character character)
    {
        Console.WriteLine($"Serializing Character: {character.Name}");
        // 这里添加实际的序列化逻辑
    }

    public void VisitEnemy(Enemy enemy)
    {
        Console.WriteLine($"Serializing Enemy with Health: {enemy.Health}");
        // 这里添加实际的序列化逻辑
    }

    public void VisitItem(Item item)
    {
        Console.WriteLine($"Serializing Item: {item.ItemName}");
        // 这里添加实际的序列化逻辑
    }
}

// 反序列化访问者
public class DeserializeVisitor : IVisitor
{
    public void VisitCharacter(Character character)
    {
        Console.WriteLine("Deserializing Character");
        // 这里添加实际的反序列化逻辑
    }

    public void VisitEnemy(Enemy enemy)
    {
        Console.WriteLine("Deserializing Enemy");
        // 这里添加实际的反序列化逻辑
    }

    public void VisitItem(Item item)
    {
        Console.WriteLine("Deserializing Item");
        // 这里添加实际的反序列化逻辑
    }
}

class Program
{
    static void Main()
    {
        IElement[] elements = new IElement[]
        {
            new Character(), new Enemy(), new Item()
        };

        SerializeVisitor serializeVisitor = new SerializeVisitor();
        DeserializeVisitor deserializeVisitor = new DeserializeVisitor();

        foreach (var element in elements)
        {
            element.Accept(serializeVisitor);
        }

        foreach (var element in elements)
        {
            element.Accept(deserializeVisitor);
        }
    }
}

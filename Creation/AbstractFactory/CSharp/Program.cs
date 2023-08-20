using System;

namespace AbstractFactoryExample
{
    public interface IWeapon
    {
        string Describe();
    }

    public interface IEquipment
    {
        string Describe();
    }

    // 刀剣
    public class Sword : IWeapon
    {
        public string Describe() => "Sword";
    }

    // 弓
    public class Bow : IWeapon
    {
        public string Describe() => "Bow";
    }

    // 鎧
    public class Armor : IEquipment
    {
        public string Describe() => "Armor";
    }

    // 矢筒
    public class Quiver : IEquipment
    {
        public string Describe() => "Quiver";
    }

    public interface IGameFactory
    {
        IWeapon CreateWeapon();
        IEquipment CreateEquipment();
    }

    // 戦士
    public class WarriorFactory : IGameFactory
    {
        public IWeapon CreateWeapon() => new Sword();
        public IEquipment CreateEquipment() => new Armor();
    }

    // 弓使い
    public class ArcherFactory : IGameFactory
    {
        public IWeapon CreateWeapon() => new Bow();
        public IEquipment CreateEquipment() => new Quiver();
    }

    class Program
    {
        static void Main(string[] args)
        {
            IGameFactory warriorFactory = new WarriorFactory();
            var warriorWeapon = warriorFactory.CreateWeapon();
            var warriorEquipment = warriorFactory.CreateEquipment();
            Console.WriteLine($"Warrior uses {warriorWeapon.Describe()} and wears {warriorEquipment.Describe()}.");

            IGameFactory archerFactory = new ArcherFactory();
            var archerWeapon = archerFactory.CreateWeapon();
            var archerEquipment = archerFactory.CreateEquipment();
            Console.WriteLine($"Archer uses {archerWeapon.Describe()} and wears {archerEquipment.Describe()}.");
        }
    }
}

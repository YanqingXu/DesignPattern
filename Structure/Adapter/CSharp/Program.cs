using System;

namespace AdapterPattern
{
    // Target Interface
    public interface IStorage
    {
        void Save(string data);
        string Load();
    }

    // Adaptee: XMLStorage
    public class XMLStorage
    {
        public void SaveToXML(string data)
        {
            Console.WriteLine($"Data saved to XML: {data}");
        }

        public string LoadFromXML()
        {
            return "Data loaded from XML";
        }
    }

    // Adapter for XMLStorage
    public class XMLStorageAdapter : IStorage
    {
        private XMLStorage _xmlStorage = new XMLStorage();

        public void Save(string data)
        {
            _xmlStorage.SaveToXML(data);
        }

        public string Load()
        {
            return _xmlStorage.LoadFromXML();
        }
    }

    // Adaptee: DatabaseStorage
    public class DatabaseStorage
    {
        public void SaveToDatabase(string data)
        {
            Console.WriteLine($"Data saved to Database: {data}");
        }

        public string LoadFromDatabase()
        {
            return "Data loaded from Database";
        }
    }

    // Adapter for DatabaseStorage
    public class DatabaseStorageAdapter : IStorage
    {
        private DatabaseStorage _dbStorage = new DatabaseStorage();

        public void Save(string data)
        {
            _dbStorage.SaveToDatabase(data);
        }

        public string Load()
        {
            return _dbStorage.LoadFromDatabase();
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            IStorage storage;

            // Use XML storage
            storage = new XMLStorageAdapter();
            storage.Save("Sample Data");
            Console.WriteLine(storage.Load());

            // Use Database storage
            storage = new DatabaseStorageAdapter();
            storage.Save("Sample Data");
            Console.WriteLine(storage.Load());
        }
    }
}

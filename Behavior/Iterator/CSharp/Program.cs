using System;
using System.Collections;
using System.Collections.Generic;

// 定义一个简单的集合类
public class CustomCollection : IEnumerable<int>
{
    private List<int> _items = new List<int>();

    public void Add(int item)
    {
        _items.Add(item);
    }

    // 返回一个IEnumerator<int>接口实现
    public IEnumerator<int> GetEnumerator()
    {
        return new CustomIterator(this);
    }

    // 显式接口实现，返回非泛型的IEnumerator接口实现
    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }

    // 获取集合的大小
    public int Count
    {
        get { return _items.Count; }
    }

    // 索引器
    public int this[int index]
    {
        get { return _items[index]; }
    }
}


// 定义迭代器类
public class CustomIterator : IEnumerator<int>
{
    private readonly CustomCollection _collection;
    private int _currentIndex = -1; // 迭代器初始位置之前

    public CustomIterator(CustomCollection collection)
    {
        _collection = collection;
    }

    public bool MoveNext()
    {
        _currentIndex++;
        return _currentIndex < _collection.Count;
    }

    public void Reset()
    {
        _currentIndex = -1;
    }

    public int Current
    {
        get
        {
            if (_currentIndex == -1 || _currentIndex == _collection.Count)
                throw new InvalidOperationException();
            return _collection[_currentIndex];
        }
    }

    // 显式接口实现，返回的是object类型
    object IEnumerator.Current
    {
        get { return Current; }
    }

    public void Dispose()
    {
        // 如果需要释放资源，可以在这里实现
    }
}


class Program
{
    static void Main(string[] args)
    {
        var collection = new CustomCollection();
        collection.Add(1);
        collection.Add(2);
        collection.Add(3);

        foreach (var item in collection)
        {
            Console.WriteLine(item);
        }
    }
}

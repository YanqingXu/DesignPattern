迭代器模式（Iterator Pattern）是一种常用的设计模式，属于行为型模式的一种。这个模式用于顺序访问集合对象的元素，不需要了解集合对象的底层表示。

### 目的：
迭代器模式的主要目的是提供一种方法顺序访问一个集合对象中各个元素，而又不暴露其内部的表示。
迭代器模式可以把迭代逻辑从集合中分离出来，减少集合类的职责，同时也提供了一种统一的接口遍历各种类型的集合，增加了程序的可复用性。

### 主要角色：
1. **迭代器（Iterator）角色**：该接口定义了访问和遍历元素的接口，通常包括`hasNext()`, `next()`, `remove()`等方法。
2. **具体迭代器（ConcreteIterator）角色**：实现迭代器接口，并保持追踪当前遍历的位置。
3. **容器（Collection）角色**：定义创建迭代器的接口，它返回一个新的迭代器实例。
4. **具体容器（ConcreteCollection）角色**：实现创建迭代器的接口，返回与该集合相对应的具体迭代器实例。

### 优点：
- 支持以不同的方式遍历一个集合
- 迭代器简化了集合的接口
- 在同一个集合上可以有多个遍历
- 在迭代器模式中，增加新的集合类和迭代器类都很方便，无需修改原有代码

### 缺点：
- 对于比较简单的遍历（如在数组上的正向遍历），使用迭代器模式可能会略显笨重

### 应用场景：
- 当你的集合对象有复杂的数据结构，且你希望隐藏其复杂结构的实现细节时
- 当你需要对集合对象提供多种遍历方式时
- 当你希望有一个统一的接口遍历各种类型的集合，而不希望暴露其内部结构时

### 实现示例（Java代码）：

```java
// 迭代器接口
public interface Iterator<T> {
    boolean hasNext();
    T next();
}

// 具体迭代器
public class ConcreteIterator<T> implements Iterator<T> {
    private List<T> collection;
    private int pos = 0;

    public ConcreteIterator(List<T> collection) {
        this.collection = collection;
    }

    @Override
    public boolean hasNext() {
        return pos < collection.size();
    }

    @Override
    public T next() {
        if (this.hasNext()) {
            return collection.get(pos++);
        }
        return null;
    }
}

// 集合接口
public interface Collection<T> {
    Iterator<T> iterator();
    // 其他集合相关的方法...
}

// 具体集合
public class ConcreteCollection<T> implements Collection<T> {
    private List<T> list = new ArrayList<>();

    // 添加元素到集合
    public void add(T obj) {
        list.add(obj);
    }

    // 创建迭代器
    @Override
    public Iterator<T> iterator() {
        return new ConcreteIterator<>(list);
    }
}
```

### 使用：

```java
Collection<String> collection = new ConcreteCollection<>();
collection.add("item1");
collection.add("item2");
collection.add("item3");

Iterator<String> iterator = collection.iterator();
while(iterator.hasNext()) {
    String item = iterator.next();
    System.out.println(item);
}
```

在实际应用中，许多编程语言的标准库，如 Java 的 `Collection` 接口，已经内置了迭代器模式的支持。开发者可以直接使用而无需自己实现。
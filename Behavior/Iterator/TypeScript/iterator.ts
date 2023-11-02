// 迭代器接口定义
interface Iterator<T> {
    next(): IteratorResults<T>;
    hasNext(): boolean;
}

// 迭代结果接口定义
interface IteratorResults<T> {
    value: T | null;
    done: boolean;
}

// 可迭代对象接口定义
interface Iterable<T> {
    [Symbol.iterator](): Iterator<T>;
}

// 具体的迭代器实现
class ConcreteIterator<T> implements Iterator<T> {
    private collection: T[];
    private index: number = 0;

    constructor(collection: T[]) {
        this.collection = collection;
    }

    public next(): IteratorResults<T> {
        if (this.hasNext()) {
            return { value: this.collection[this.index++], done: false };
        } else {
            return { value: null, done: true };
        }
    }

    public hasNext(): boolean {
        return this.index < this.collection.length;
    }
}

// 具体的可迭代对象实现
class ConcreteAggregate<T> implements Iterable<T> {
    private items: T[] = [];

    constructor(items: T[]) {
        this.items = items;
    }

    public [Symbol.iterator](): Iterator<T> {
        return new ConcreteIterator(this.items);
    }
}

// 使用迭代器模式
const collection = new ConcreteAggregate<number>([1, 2, 3, 4, 5]);
const iterator = collection[Symbol.iterator]();

while (iterator.hasNext()) {
    console.log(iterator.next().value);
}

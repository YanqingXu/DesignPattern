package main

import "fmt"

// Iterator 是迭代器接口
type Iterator interface {
	HasNext() bool
	Next() interface{}
}

// IntSliceIterator 是int切片的迭代器
type IntSliceIterator struct {
	slice []int
	index int
}

// HasNext 方法检查迭代器是否已到达切片的末尾
func (i *IntSliceIterator) HasNext() bool {
	return i.index < len(i.slice)
}

// Next 方法返回切片中的下一个元素，并将索引向前移动
func (i *IntSliceIterator) Next() interface{} {
	if i.HasNext() {
		value := i.slice[i.index]
		i.index++
		return value
	}
	return nil // 或者返回错误
}

// IterableIntSlice 是一个可以迭代的int切片
type IterableIntSlice []int

// CreateIterator 创建一个IntSliceIterator来遍历切片
func (s IterableIntSlice) CreateIterator() Iterator {
	return &IntSliceIterator{
		slice: s,
		index: 0,
	}
}

func main() {
	slice := IterableIntSlice{1, 2, 3, 4, 5}
	it := slice.CreateIterator()
	for it.HasNext() {
		value := it.Next()
		fmt.Println(value)
	}
}

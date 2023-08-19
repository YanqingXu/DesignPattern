package main

import (
	"fmt"
)

// Prototype 接口
type Prototype interface {
	Clone() Prototype
	Operation()
}

// ConcretePrototype 结构体
type ConcretePrototype struct {
	data int
}

// Clone 方法实现
func (cp *ConcretePrototype) Clone() Prototype {
	return &ConcretePrototype{data: cp.data}
}

// Operation 方法实现
func (cp *ConcretePrototype) Operation() {
	fmt.Printf("ConcretePrototype operation with data: %d\n", cp.data)
}

func main() {
	// 创建一个具体原型对象
	prototype := &ConcretePrototype{data: 10}

	// 克隆原型对象
	clonedPrototype := prototype.Clone()

	// 调用原型和克隆对象的操作
	prototype.Operation()
	clonedPrototype.Operation()
}

/*
	我们定义了一个Prototype接口，它有一个Clone方法用于创建对象的副本。
	ConcretePrototype结构体实现了这个接口，并提供了Clone方法的具体实现。

	在main函数中，我们创建了一个ConcretePrototype对象，并使用Clone方法创建了它的一个副本。
	然后，我们调用了原型和克隆对象的Operation方法，以验证它们都是有效的对象。
*/

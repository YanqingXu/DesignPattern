package main

import "fmt"

// Product 接口，代表抽象产品
type Product interface {
	Show()
}

// ConcreteProductA 具体产品A
type ConcreteProductA struct{}

func (p *ConcreteProductA) Show() {
	fmt.Println("Product A")
}

// ConcreteProductB 具体产品B
type ConcreteProductB struct{}

func (p *ConcreteProductB) Show() {
	fmt.Println("Product B")
}

// Factory 接口，代表抽象工厂
type Factory interface {
	CreateProduct() Product
}

// ConcreteFactoryA 具体工厂A
type ConcreteFactoryA struct{}

func (f *ConcreteFactoryA) CreateProduct() Product {
	return &ConcreteProductA{}
}

// ConcreteFactoryB 具体工厂B
type ConcreteFactoryB struct{}

func (f *ConcreteFactoryB) CreateProduct() Product {
	return &ConcreteProductB{}
}

func main() {
	// 使用具体工厂A创建产品A
	factoryA := &ConcreteFactoryA{}
	productA := factoryA.CreateProduct()
	productA.Show()

	// 使用具体工厂B创建产品B
	factoryB := &ConcreteFactoryB{}
	productB := factoryB.CreateProduct()
	productB.Show()
}

package main

import "fmt"

type Implementor interface {
	operationImpl()
}

type Abstraction interface {
	operation()
}

type RefinedAbstraction struct {
	implementor Implementor
}

func (ra *RefinedAbstraction) operation() {
	ra.implementor.operationImpl()
}

type ConcreteImplementorA struct{}

func (cia *ConcreteImplementorA) operationImpl() {
	fmt.Println("ConcreteImplementorA operation")
}

type ConcreteImplementorB struct{}

func (cib *ConcreteImplementorB) operationImpl() {
	fmt.Println("ConcreteImplementorB operation")
}

func main() {
	absA := &RefinedAbstraction{implementor: &ConcreteImplementorA{}}
	absA.operation()

	absB := &RefinedAbstraction{implementor: &ConcreteImplementorB{}}
	absB.operation()
}

package main

import "fmt"

type Component interface {
	Operation()
}

type Leaf struct {
}

func (l *Leaf) Operation() {
	// Leaf operation implementation
	fmt.Println("Leaf operation")
}

type Composite struct {
	components []Component
}

func (c *Composite) Operation() {
	for _, component := range c.components {
		component.Operation()
	}
}

func main() {
	composite := &Composite{}
	composite.components = append(composite.components, &Leaf{})
	composite.components = append(composite.components, &Leaf{})
	composite.components = append(composite.components, &Leaf{})
	composite.Operation()
}

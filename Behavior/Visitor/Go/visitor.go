package main

import "fmt"

// Element 接口
type Element interface {
	Accept(visitor Visitor)
}

// Character 结构体
type Character struct{}

func (c *Character) Accept(visitor Visitor) {
	visitor.VisitCharacter(c)
}

// Enemy 结构体
type Enemy struct{}

func (e *Enemy) Accept(visitor Visitor) {
	visitor.VisitEnemy(e)
}

// Item 结构体
type Item struct{}

func (i *Item) Accept(visitor Visitor) {
	visitor.VisitItem(i)
}

// Visitor 接口
type Visitor interface {
	VisitCharacter(*Character)
	VisitEnemy(*Enemy)
	VisitItem(*Item)
}

// RenderVisitor 结构体实现了 Visitor 接口
type RenderVisitor struct{}

func (rv *RenderVisitor) VisitCharacter(c *Character) {
	fmt.Println("Rendering a Character")
}

func (rv *RenderVisitor) VisitEnemy(e *Enemy) {
	fmt.Println("Rendering an Enemy")
}

func (rv *RenderVisitor) VisitItem(i *Item) {
	fmt.Println("Rendering an Item")
}

func main() {
	elements := []Element{&Character{}, &Enemy{}, &Item{}}

	renderVisitor := &RenderVisitor{}

	for _, element := range elements {
		element.Accept(renderVisitor)
	}
}

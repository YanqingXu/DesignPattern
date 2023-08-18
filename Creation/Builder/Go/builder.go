package main

import "fmt"

// 1. Product
type Car struct {
	Wheels int
	Color  string
	Engine string
}

func (c *Car) ShowDetails() {
	fmt.Printf("Car with %d wheels, color %s and %s engine.\n", c.Wheels, c.Color, c.Engine)
}

// 2. Builder
type CarBuilder interface {
	SetWheels() CarBuilder
	SetColor() CarBuilder
	SetEngine() CarBuilder
	Build() *Car
}

// 3. ConcreteBuilder
type SedanCarBuilder struct {
	car *Car
}

func NewSedanCarBuilder() *SedanCarBuilder {
	return &SedanCarBuilder{&Car{}}
}

func (b *SedanCarBuilder) SetWheels() CarBuilder {
	b.car.Wheels = 4
	return b
}

func (b *SedanCarBuilder) SetColor() CarBuilder {
	b.car.Color = "Red"
	return b
}

func (b *SedanCarBuilder) SetEngine() CarBuilder {
	b.car.Engine = "V6"
	return b
}

func (b *SedanCarBuilder) Build() *Car {
	return b.car
}

// 4. Director
type ManufacturingDirector struct {
	builder CarBuilder
}

func (d *ManufacturingDirector) SetBuilder(b CarBuilder) {
	d.builder = b
}

func (d *ManufacturingDirector) Construct() {
	d.builder.SetWheels().SetColor().SetEngine()
}

func main() {
	// Client code
	director := &ManufacturingDirector{}
	sedanBuilder := NewSedanCarBuilder()

	director.SetBuilder(sedanBuilder)
	director.Construct()

	car := sedanBuilder.Build()
	car.ShowDetails()
}

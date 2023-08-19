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
	SetWheels(int) CarBuilder
	SetColor(string) CarBuilder
	SetEngine(string) CarBuilder
	Build() *Car
}

// 3. ConcreteBuilder
type SedanCarBuilder struct {
	car *Car
}

func NewSedanCarBuilder() *SedanCarBuilder {
	return &SedanCarBuilder{&Car{}}
}

func (b *SedanCarBuilder) SetWheels(wheels int) CarBuilder {
	b.car.Wheels = wheels
	return b
}

func (b *SedanCarBuilder) SetColor(color string) CarBuilder {
	b.car.Color = color
	return b
}

func (b *SedanCarBuilder) SetEngine(engine string) CarBuilder {
	b.car.Engine = engine
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

func (d *ManufacturingDirector) Construct(wheels int, color string, engine string) {
	d.builder.SetWheels(wheels).SetColor(color).SetEngine(engine)
}

func main() {
	// Client code
	director := &ManufacturingDirector{}
	sedanBuilder := NewSedanCarBuilder()

	director.SetBuilder(sedanBuilder)
	director.Construct(4, "Red", "V8")

	car := sedanBuilder.Build()
	car.ShowDetails()
}

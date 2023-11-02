package main

import "fmt"

// Mediator interface
type Mediator interface {
	Send(message string, colleague Colleague)
}

// Colleague interface
type Colleague interface {
	Send(message string)
	Receive(message string)
	SetMediator(mediator Mediator)
}

// ConcreteMediator is a mediator for Colleague objects
type ConcreteMediator struct {
	colleague1 Colleague
	colleague2 Colleague
}

// Send message to all colleagues except the sender
func (m *ConcreteMediator) Send(message string, sender Colleague) {
	if m.colleague1 == sender {
		m.colleague2.Receive(message)
	} else {
		m.colleague1.Receive(message)
	}
}

// SetColleague1 sets the first colleague
func (m *ConcreteMediator) SetColleague1(colleague Colleague) {
	m.colleague1 = colleague
}

// SetColleague2 sets the second colleague
func (m *ConcreteMediator) SetColleague2(colleague Colleague) {
	m.colleague2 = colleague
}

// ConcreteColleague1 is a concrete colleague
type ConcreteColleague1 struct {
	mediator Mediator
}

// Send message through mediator
func (c *ConcreteColleague1) Send(message string) {
	c.mediator.Send(message, c)
}

// Receive message from mediator
func (c *ConcreteColleague1) Receive(message string) {
	fmt.Println("Colleague1 Received:", message)
}

// SetMediator sets the mediator
func (c *ConcreteColleague1) SetMediator(mediator Mediator) {
	c.mediator = mediator
}

// ConcreteColleague2 is a concrete colleague
type ConcreteColleague2 struct {
	mediator Mediator
}

// Send message through mediator
func (c *ConcreteColleague2) Send(message string) {
	c.mediator.Send(message, c)
}

// Receive message from mediator
func (c *ConcreteColleague2) Receive(message string) {
	fmt.Println("Colleague2 Received:", message)
}

// SetMediator sets the mediator
func (c *ConcreteColleague2) SetMediator(mediator Mediator) {
	c.mediator = mediator
}

// main function
func main() {
	mediator := &ConcreteMediator{}

	colleague1 := &ConcreteColleague1{}
	colleague1.SetMediator(mediator)
	colleague2 := &ConcreteColleague2{}
	colleague2.SetMediator(mediator)

	mediator.SetColleague1(colleague1)
	mediator.SetColleague2(colleague2)

	colleague1.Send("Hi there!")
	colleague2.Send("Hello!")
}

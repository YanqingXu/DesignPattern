package main

import "fmt"

// Memento 是一个存储原发器状态的结构体
type Memento struct {
	state string
}

// GetState 用于获取存储的状态
func (m *Memento) GetState() string {
	return m.state
}

// Originator 是原发器结构体，它的状态需要被保存和恢复
type Originator struct {
	State string
}

// NewMemento 创建一个新的备忘录，用于保存当前状态
func (o *Originator) NewMemento() *Memento {
	return &Memento{state: o.State}
}

// Restore 从备忘录恢复状态
func (o *Originator) Restore(m *Memento) {
	o.State = m.GetState()
}

// Caretaker 是管理者结构体，负责保存备忘录
type Caretaker struct {
	mementoList []*Memento
}

// AddMemento 添加备忘录到列表
func (c *Caretaker) AddMemento(m *Memento) {
	c.mementoList = append(c.mementoList, m)
}

// GetMemento 获取备忘录
func (c *Caretaker) GetMemento(index int) *Memento {
	if index < len(c.mementoList) {
		return c.mementoList[index]
	}
	return nil
}

func main() {
	originator := Originator{}
	caretaker := Caretaker{}

	originator.State = "On"
	// 保存状态
	caretaker.AddMemento(originator.NewMemento())

	// 更改状态
	originator.State = "Off"

	// 恢复到之前的状态
	originator.Restore(caretaker.GetMemento(0))
	fmt.Println("Restored State:", originator.State)
}

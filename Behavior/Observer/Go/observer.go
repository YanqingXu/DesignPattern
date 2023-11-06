package main

import (
	"fmt"
)

// Observer 接口定义了观察者需要实现的更新方法
type Observer interface {
	Update(message string)
}

// Subject 接口定义了被观察者需要实现的方法，以便观察者可以注册和注销
type Subject interface {
	RegisterObserver(o Observer)
	DeregisterObserver(o Observer)
	NotifyObservers()
}

// TeachingSystem 是实现了Subject接口的教学系统
type TeachingSystem struct {
	observers []Observer
	message   string
}

// RegisterObserver 用于注册新的观察者
func (t *TeachingSystem) RegisterObserver(o Observer) {
	t.observers = append(t.observers, o)
}

// DeregisterObserver 用于注销现有的观察者
func (t *TeachingSystem) DeregisterObserver(o Observer) {
	var indexToRemove int
	for i, observer := range t.observers {
		if observer == o {
			indexToRemove = i
			break
		}
	}
	t.observers = append(t.observers[:indexToRemove], t.observers[indexToRemove+1:]...)
}

// NotifyObservers 通知所有注册的观察者
func (t *TeachingSystem) NotifyObservers() {
	for _, observer := range t.observers {
		observer.Update(t.message)
	}
}

// SetMessage 改变教学系统的消息，并通知观察者
func (t *TeachingSystem) SetMessage(message string) {
	t.message = message
	t.NotifyObservers()
}

// TipDisplay 是一个观察者，它在用户界面中显示提示
type TipDisplay struct {
	// 可以包含一些特定于显示的字段
}

// Update 是TipDisplay的Observer接口实现
func (td *TipDisplay) Update(message string) {
	fmt.Printf("Tip Display: %s\n", message)
}

func main() {
	// 创建教学系统
	teachingSystem := &TeachingSystem{}

	// 创建提示显示观察者
	tipDisplay := &TipDisplay{}

	// 注册观察者到教学系统
	teachingSystem.RegisterObserver(tipDisplay)

	// 更改系统状态并通知观察者
	teachingSystem.SetMessage("Remember to save your work frequently!")

	// 当提示不再需要显示时，注销观察者
	teachingSystem.DeregisterObserver(tipDisplay)

	// 更新系统状态，此时不会通知先前的观察者
	teachingSystem.SetMessage("Use comments to explain complex code logic.")
}

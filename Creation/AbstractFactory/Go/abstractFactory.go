package main

import "fmt"

// Button 接口
type Button interface {
	Display() string
}

// Background 接口
type Background interface {
	Display() string
}

// GameSkinFactory 接口
type GameSkinFactory interface {
	CreateButton() Button
	CreateBackground() Background
}

// ClassicButton 结构体
type ClassicButton struct{}

func (c *ClassicButton) Display() string {
	return "Classic Button"
}

// ClassicBackground 结构体
type ClassicBackground struct{}

func (c *ClassicBackground) Display() string {
	return "Classic Background"
}

// ModernButton 结构体
type ModernButton struct{}

func (m *ModernButton) Display() string {
	return "Modern Button"
}

// ModernBackground 结构体
type ModernBackground struct{}

func (m *ModernBackground) Display() string {
	return "Modern Background"
}

// ClassicFactory 结构体
type ClassicFactory struct{}

func (c *ClassicFactory) CreateButton() Button {
	return &ClassicButton{}
}

func (c *ClassicFactory) CreateBackground() Background {
	return &ClassicBackground{}
}

// ModernFactory 结构体
type ModernFactory struct{}

func (m *ModernFactory) CreateButton() Button {
	return &ModernButton{}
}

func (m *ModernFactory) CreateBackground() Background {
	return &ModernBackground{}
}

func main() {
	var factory GameSkinFactory

	// 使用 ClassicFactory
	factory = &ClassicFactory{}
	button := factory.CreateButton()
	background := factory.CreateBackground()
	fmt.Println(button.Display())
	fmt.Println(background.Display())

	// 使用 ModernFactory
	factory = &ModernFactory{}
	button = factory.CreateButton()
	background = factory.CreateBackground()
	fmt.Println(button.Display())
	fmt.Println(background.Display())
}

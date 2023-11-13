package main

import (
	"fmt"
)

// GameProcess 接口定义了游戏流程的基本步骤
type GameProcess interface {
	Initialize()
	Start()
	End()
}

// GameTemplate 结构体包含模板方法
type GameTemplate struct {
	GameProcess
}

// Play 是模板方法，定义了游戏流程的顺序
func (g *GameTemplate) Play() {
	g.Initialize()
	g.Start()
	g.End()
}

// AdventureGame 是一个具体的游戏流程
type AdventureGame struct{}

// 实现 GameProcess 的方法
func (a *AdventureGame) Initialize() {
	fmt.Println("Initializing Adventure Game")
}

func (a *AdventureGame) Start() {
	fmt.Println("Starting Adventure Game")
}

func (a *AdventureGame) End() {
	fmt.Println("Ending Adventure Game")
}

// ActionGame 是另一个具体的游戏流程
type ActionGame struct{}

// 实现 GameProcess 的方法
func (a *ActionGame) Initialize() {
	fmt.Println("Initializing Action Game")
}

func (a *ActionGame) Start() {
	fmt.Println("Starting Action Game")
}

func (a *ActionGame) End() {
	fmt.Println("Ending Action Game")
}

func main() {
	adventureGame := &GameTemplate{&AdventureGame{}}
	adventureGame.Play()

	actionGame := &GameTemplate{&ActionGame{}}
	actionGame.Play()
}

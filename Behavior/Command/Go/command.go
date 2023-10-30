package main

import (
	"fmt"
)

// Command 接口定义命令的执行和撤销操作
type Command interface {
	Execute()
	Undo()
}

// Character 类表示游戏中的角色
type Character struct {
	position int
}

func (c *Character) Move(distance int) {
	c.position += distance
	fmt.Printf("Moved %d units. Current position: %d\n", distance, c.position)
}

// MoveCommand 表示移动命令
type MoveCommand struct {
	character *Character
	distance   int
}

func (m *MoveCommand) Execute() {
	m.character.Move(m.distance)
}

func (m *MoveCommand) Undo() {
	m.character.Move(-m.distance)
}

// CommandInvoker 类用于执行命令
type CommandInvoker struct {
	commands []Command
	current  int
}

func (ci *CommandInvoker) Execute(command Command) {
	if ci.current < len(ci.commands) {
		ci.commands = ci.commands[:ci.current]
	}
	ci.commands = append(ci.commands, command)
	ci.current++
	command.Execute()
}

func (ci *CommandInvoker) Undo() {
	if ci.current > 0 {
		ci.current--
		ci.commands[ci.current].Undo()
	}
}

func (ci *CommandInvoker) Redo() {
	if ci.current < len(ci.commands) {
		ci.commands[ci.current].Execute()
		ci.current++
	}
}

func main() {
	player := &Character{}
	invoker := &CommandInvoker{}

	moveCommand1 := &MoveCommand{character: player, distance: 5}
	moveCommand2 := &MoveCommand{character: player, distance: -3}

	// 执行命令
	invoker.Execute(moveCommand1)
	invoker.Execute(moveCommand2)

	// 撤销上一个命令
	invoker.Undo()

	// 重做命令
	invoker.Redo()
}

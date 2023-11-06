package main

import "fmt"

// State 接口
type State interface {
	Handle(c *Context)
}

// Context 结构体，代表拥有状态的实体
type Context struct {
	State State
}

// 实现状态转换的方法
func (c *Context) Request() {
	c.State.Handle(c)
}

// 具体状态: StartState
type StartState struct{}

func (s *StartState) Handle(c *Context) {
	fmt.Println("Player is in start state")
	c.State = &StopState{}
}

// 具体状态: StopState
type StopState struct{}

func (s *StopState) Handle(c *Context) {
	fmt.Println("Player is in stop state")
	// 这里不再转换状态，表示结束
}

func main() {
	context := &Context{State: &StartState{}}

	// 应用初始状态的行为
	context.Request()

	// 状态从 StartState 转变到 StopState
	context.Request()

	// 再次调用，但状态是 StopState，应该没有变化
	context.Request()
}

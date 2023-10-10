package main

import "fmt"

// Task represents a task in the task system.
type Task struct {
	ID       int
	Type     string
	Progress int
}

// TaskHandler defines the interface for handling tasks.
type TaskHandler interface {
	HandleTask(*Task)
	SetNext(TaskHandler)
}

// BaseTaskHandler is the base struct for implementing TaskHandler.
type BaseTaskHandler struct {
	next TaskHandler
}

func (h *BaseTaskHandler) SetNext(next TaskHandler) {
	h.next = next
}

// ConcreteTaskHandlerA is a concrete task handler for handling type A tasks.
type ConcreteTaskHandlerA struct {
	BaseTaskHandler
}

func (h *ConcreteTaskHandlerA) HandleTask(task *Task) {
	if task.Type == "A" {
		fmt.Printf("Handling Type A Task %d\n", task.ID)
		// Handle the task logic for Type A here...
		task.Progress++
	} else if h.next != nil {
		h.next.HandleTask(task)
	}
}

// ConcreteTaskHandlerB is a concrete task handler for handling type B tasks.
type ConcreteTaskHandlerB struct {
	BaseTaskHandler
}

func (h *ConcreteTaskHandlerB) HandleTask(task *Task) {
	if task.Type == "B" {
		fmt.Printf("Handling Type B Task %d\n", task.ID)
		// Handle the task logic for Type B here...
		task.Progress++
	} else if h.next != nil {
		h.next.HandleTask(task)
	}
}

func main() {
	// Create a chain of task handlers
	handlerA := &ConcreteTaskHandlerA{}
	handlerB := &ConcreteTaskHandlerB{}

	handlerA.SetNext(handlerB)

	// Create some sample tasks
	task1 := &Task{ID: 1, Type: "A", Progress: 0}
	task2 := &Task{ID: 2, Type: "B", Progress: 0}
	task3 := &Task{ID: 3, Type: "C", Progress: 0}

	// Handle the tasks through the chain
	handlerA.HandleTask(task1)
	handlerA.HandleTask(task2)
	handlerA.HandleTask(task3)
}

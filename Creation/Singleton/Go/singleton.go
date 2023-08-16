package main

import (
	"fmt"
	"sync"
)

type Singleton[T any] struct {
	instance *T
	once     sync.Once
}

func (s *Singleton[T]) GetInstance(defaultValue func() T) *T {
	s.once.Do(func() {
		instance := defaultValue()
		s.instance = &instance
	})
	return s.instance
}

func main() {
	var intSingleton Singleton[int]
	var stringSingleton Singleton[string]

	intInstance1 := intSingleton.GetInstance(func() int { return 42 })
	intInstance2 := intSingleton.GetInstance(func() int { return 43 })

	if intInstance1 == intInstance2 {
		fmt.Println("两个int实例是相同的")
	} else {
		fmt.Println("两个int实例是不同的")
	}

	stringInstance1 := stringSingleton.GetInstance(func() string { return "Hello" })
	stringInstance2 := stringSingleton.GetInstance(func() string { return "World" })

	if stringInstance1 == stringInstance2 {
		fmt.Println("两个string实例是相同的")
	} else {
		fmt.Println("两个string实例是不同的")
	}
}

/*
在这个实现中，Singleton 结构体使用了泛型 T。
GetInstance 方法接受一个函数 defaultValue，该函数返回类型为 T 的值。
这个函数只会在第一次调用 GetInstance 时被执行，以初始化单例的值。
*/

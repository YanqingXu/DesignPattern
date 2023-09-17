package main

import "fmt"

// 定义一个函数类型
type Function func(string) string

// 定义一个装饰器
func Decorator(f Function) Function {
	return func(name string) string {
		fmt.Println("Starting...")
		result := f(name)
		fmt.Println("Ended...")
		return result
	}
}

// 定义一个函数
func Hello(name string) string {
	return fmt.Sprintf("Hello, %s!", name)
}

func main() {
	// 使用装饰器装饰函数
	DecoratedHello := Decorator(Hello)
	// 调用装饰后的函数
	fmt.Println(DecoratedHello("Gopher"))
}

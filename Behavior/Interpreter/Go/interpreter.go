package main

import (
	"fmt"
	"strconv"
	"strings"
)

// Expression 接口声明了解释操作
type Expression interface {
	Interpret() int
}

// NumberExpression 是数字表达式
type NumberExpression struct {
	number int
}

func (ne *NumberExpression) Interpret() int {
	return ne.number
}

// PlusExpression 是加法表达式
type PlusExpression struct {
	left  Expression
	right Expression
}

func (pe *PlusExpression) Interpret() int {
	return pe.left.Interpret() + pe.right.Interpret()
}

// MinusExpression 是减法表达式
type MinusExpression struct {
	left  Expression
	right Expression
}

func (me *MinusExpression) Interpret() int {
	return me.left.Interpret() - me.right.Interpret()
}

// Parser 将解析输入并返回适当的表达式
func Parser(input string) Expression {
	parts := strings.Split(input, " ")

	switch parts[1] {
	case "+":
		left, _ := strconv.Atoi(parts[0])
		right, _ := strconv.Atoi(parts[2])
		return &PlusExpression{
			left:  &NumberExpression{number: left},
			right: &NumberExpression{number: right},
		}
	case "-":
		left, _ := strconv.Atoi(parts[0])
		right, _ := strconv.Atoi(parts[2])
		return &MinusExpression{
			left:  &NumberExpression{number: left},
			right: &NumberExpression{number: right},
		}
	default:
		number, _ := strconv.Atoi(parts[0])
		return &NumberExpression{number: number}
	}
}

func main() {
	input := "3 + 5"
	expression := Parser(input)
	result := expression.Interpret()
	fmt.Printf("%s = %d\n", input, result) // 3 + 5 = 8

	input = "10 - 6"
	expression = Parser(input)
	result = expression.Interpret()
	fmt.Printf("%s = %d\n", input, result) // 10 - 6 = 4
}

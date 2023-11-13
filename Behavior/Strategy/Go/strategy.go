package main

import "fmt"

// WeaponBehavior 策略接口
type WeaponBehavior interface {
	UseWeapon()
}

// KnifeBehavior 实现了近战攻击
type KnifeBehavior struct{}

func (KnifeBehavior) UseWeapon() {
	fmt.Println("Performing an attack with a knife")
}

// GunBehavior 实现了射击攻击
type GunBehavior struct{}

func (GunBehavior) UseWeapon() {
	fmt.Println("Performing an attack with a gun")
}

// Character 角色类，使用武器策略
type Character struct {
	weapon WeaponBehavior
}

func (c *Character) SetWeapon(weapon WeaponBehavior) {
	c.weapon = weapon
}

func (c *Character) Fight() {
	if c.weapon != nil {
		c.weapon.UseWeapon()
	} else {
		fmt.Println("No weapon behavior set")
	}
}

func main() {
	// 创建一个角色并设置初始武器行为
	character := Character{}
	character.SetWeapon(KnifeBehavior{})
	character.Fight() // 输出: Performing an attack with a knife

	// 改变角色的武器行为为射击
	character.SetWeapon(GunBehavior{})
	character.Fight() // 输出: Performing an attack with a gun
}

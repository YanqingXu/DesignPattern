package main

import "fmt"

// OldAudioAPI (Adaptee)
type OldAudioAPI struct{}

func (api *OldAudioAPI) PlaySoundFile(filename string) {
	fmt.Println("Playing sound using OldAudioAPI:", filename)
}

// ModernAudioAPI
type ModernAudioAPI struct{}

func (api *ModernAudioAPI) StreamAndPlay(filename string) {
	fmt.Println("Streaming and playing sound using ModernAudioAPI:", filename)
}

// AudioPlayer (Target)
type AudioPlayer interface {
	Play(filename string)
}

// OldAPIAdapter
type OldAPIAdapter struct {
	oldAPI OldAudioAPI
}

func (adapter *OldAPIAdapter) Play(filename string) {
	adapter.oldAPI.PlaySoundFile(filename)
}

// ModernAPIAdapter
type ModernAPIAdapter struct {
	modernAPI ModernAudioAPI
}

func (adapter *ModernAPIAdapter) Play(filename string) {
	adapter.modernAPI.StreamAndPlay(filename)
}

// Main function
func main() {
	var player AudioPlayer

	// Using OldAudioAPI
	player = &OldAPIAdapter{}
	player.Play("oldSound.wav")

	// Using ModernAudioAPI
	player = &ModernAPIAdapter{}
	player.Play("modernSound.mp3")
}

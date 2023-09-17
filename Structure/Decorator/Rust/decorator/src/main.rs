// 基本音频播放器
struct AudioPlayer;

impl AudioPlayer {
    fn play(&self) {
        println!("Playing audio");
    }
}

// 装饰器特质
trait AudioDecorator {
    fn play(&self);
}

// 具体装饰器 - 音效
struct SoundEffectDecorator<'a> {
    player: &'a AudioPlayer,
    sound_effect: &'a str,
}

impl<'a> SoundEffectDecorator<'a> {
    fn new(player: &'a AudioPlayer, sound_effect: &'a str) -> Self {
        SoundEffectDecorator {
            player,
            sound_effect,
        }
    }
}

impl<'a> AudioDecorator for SoundEffectDecorator<'a> {
    fn play(&self) {
        self.player.play();
        println!("Applying sound effect: {}", self.sound_effect);
    }
}

// 具体装饰器 - 音乐
struct MusicDecorator<'a> {
    player: &'a AudioPlayer,
    music: &'a str,
}

impl<'a> MusicDecorator<'a> {
    fn new(player: &'a AudioPlayer, music: &'a str) -> Self {
        MusicDecorator { player, music }
    }
}

impl<'a> AudioDecorator for MusicDecorator<'a> {
    fn play(&self) {
        self.player.play();
        println!("Playing music: {}", self.music);
    }
}

fn main() {
    // 创建基本音频播放器
    let player = AudioPlayer;

    // 使用装饰器添加音效和音乐
    let player_with_sound_effect = SoundEffectDecorator::new(&player, "Explosion");
    player_with_sound_effect.play();

    let player_with_music = MusicDecorator::new(&player, "Epic Theme");
    player_with_music.play();
}
